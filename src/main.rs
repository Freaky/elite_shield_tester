use std::error::Error;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashSet;

use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use structopt::StructOpt;

mod combinations;
mod kdtree;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ShieldGenerator {
    ship: String,
    class: u8,
    #[serde(rename = "Type")]
    kind: String,
    engineering: String,
    experimental: String,
    shield_strength: f64,
    regen_rate: f64,
    exp_res: f64,
    kin_res: f64,
    therm_res: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ShieldBooster {
    rating: char,
    engineering: String,
    experimental: String,
    shield_strength_bonus: f64,
    exp_res_bonus: f64,
    kin_res_bonus: f64,
    therm_res_bonus: f64,
}

#[derive(Debug, Clone)]
struct LoadoutStat {
    hit_points: f64,
    regen_rate: f64,
    exp_res: f64,
    kin_res: f64,
    therm_res: f64,
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "elite_shield_tester",
    about = "Elite Dangerous Shield Optimiser"
)]
struct TestConfig {
    /// Number of shield boosters to fit
    #[structopt(short, long, default_value = "1")]
    shield_booster_count: usize,
    /// Explosive damage per second
    #[structopt(short, long, default_value = "0")]
    explosive_dps: f64,
    /// Kinetic damage per second
    #[structopt(short, long, default_value = "0")]
    kinetic_dps: f64,
    /// Thermal damage per second
    #[structopt(short, long, default_value = "0")]
    thermal_dps: f64,
    /// Absolute damage per second
    #[structopt(short, long, default_value = "0")]
    absolute_dps: f64,
    /// Attacker shot success ratio, 0-1
    #[structopt(short, long, default_value = "0.5")]
    damage_effectiveness: f64,
    /// Mj available via Shield Cell Banks
    #[structopt(long, default_value = "0")]
    shield_cell_mj: f64,
    /// Mj provided by Guardian Shield Reinforcements
    #[structopt(long, default_value = "0")]
    reinforced_mj: f64,
    /// Filter out prismatic shields
    #[structopt(long)]
    disable_prismatic: bool,
    /// Disregard shields that take longer than this many seconds to regenerate from 50%
    #[structopt(long)]
    regen_time_limit: Option<f64>,
    /// Require experimental effects
    #[structopt(long)]
    force_experimental: bool,
    /// Disable pre-filtering (debugging)
    #[structopt(long)]
    disable_filter: bool,
    /// Override default shield list
    #[structopt(long)]
    shield_csv: Option<PathBuf>,
    /// Override default booster list
    #[structopt(long)]
    booster_csv: Option<PathBuf>,
    /// Ship name
    #[structopt(long, default_value = "Anaconda")]
    ship: String,
    /// Shield class (default: maximum possible)
    #[structopt(long)]
    shield_class: Option<u8>,
    /// Booster rating, A-E
    #[structopt(long, default_value = "A")]
    booster_rating: char
}

#[derive(Debug, Clone)]
struct TestResult {
    actual_dps: f64,
    survival_time: f64,
    shield: ShieldGenerator,
    boosters: Vec<ShieldBooster>,
    stats: LoadoutStat,
}

fn parse_csv<T, R>(s: R) -> Result<Vec<T>, csv::Error>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut reader = csv::Reader::from_reader(s);
    let mut ret = vec![];
    for record in reader.deserialize() {
        let record: T = record?;
        ret.push(record);
    }
    Ok(ret)
}

fn diminish_res(res: f64) -> f64 {
    if res < 0.7 {
        0.7 - (0.7 - res) / 2.0
    } else {
        res
    }
}

#[derive(Debug, Clone)]
struct BoosterStat {
    exp_modifier: f64,
    kin_modifier: f64,
    therm_modifier: f64,
    hit_point_bonus: f64,
}

fn calculate_booster_stats(boosters: &[&ShieldBooster]) -> BoosterStat {
    let mut exp_modifier = 1.0;
    let mut kin_modifier = 1.0;
    let mut therm_modifier = 1.0;
    let mut hit_point_bonus = 1.0;

    for booster in boosters.iter() {
        exp_modifier *= booster.exp_res_bonus;
        kin_modifier *= booster.kin_res_bonus;
        therm_modifier *= booster.therm_res_bonus;

        hit_point_bonus += booster.shield_strength_bonus;
    }

    BoosterStat {
        exp_modifier: diminish_res(exp_modifier),
        kin_modifier: diminish_res(kin_modifier),
        therm_modifier: diminish_res(therm_modifier),
        hit_point_bonus,
    }
}

fn calculate_loadout_stats(shield: &ShieldGenerator, boosters: &BoosterStat) -> LoadoutStat {
    LoadoutStat {
        hit_points: boosters.hit_point_bonus * shield.shield_strength,
        exp_res: shield.exp_res * boosters.exp_modifier,
        kin_res: shield.kin_res * boosters.kin_modifier,
        therm_res: shield.therm_res * boosters.therm_modifier,
        regen_rate: shield.regen_rate,
    }
}

fn calculate_actual_dps(test: &TestConfig, loadout: &LoadoutStat) -> f64 {
    test.damage_effectiveness
        * (test.explosive_dps * loadout.exp_res
            + test.kinetic_dps * loadout.kin_res
            + test.thermal_dps * loadout.therm_res
            + test.absolute_dps)
        - loadout.regen_rate * (1.0 - test.damage_effectiveness)
}

fn calculate_regen_time(loadout: &LoadoutStat) -> f64 {
    (loadout.hit_points / 2.0) / loadout.regen_rate
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = TestConfig::from_args();

    println!(
        "Elite Shield Tester Rust Edition v{}",
        env!("CARGO_PKG_VERSION")
    );

    let mut generators: Vec<ShieldGenerator> =
        parse_csv(&include_bytes!("../data/Shields.csv")[..])?;
    let mut boosters: Vec<ShieldBooster> =
        parse_csv(&include_bytes!("../data/ShieldBoosterVariants.csv")[..])?;

    if let Some(ref path) = test.shield_csv {
        println!("Custom Shield CSV: {}", path.display());
        generators = parse_csv(std::fs::File::open(path)?)?;
    }

    if let Some(ref path) = test.booster_csv {
        println!("Custom Booster CSV: {}", path.display());
        boosters = parse_csv(std::fs::File::open(path)?)?;
    }

    let total_boosters = boosters.len();
    let boosters: Vec<ShieldBooster> = boosters
        .into_iter()
        .filter(|booster| booster.rating == test.booster_rating.to_ascii_uppercase())
        .filter(|booster| {
            !test.force_experimental || booster.experimental != "No Experimental Effect"
        })
        .filter(|booster| {
            // Naively filter out irrelevant boosters
            test.disable_filter
                || !(test.explosive_dps == 0.0 && (booster.engineering == "Blast Resistance")
                    || test.kinetic_dps == 0.0 && (booster.engineering == "Kinetic Resistance")
                    || test.thermal_dps == 0.0 && (booster.engineering == "Thermal Resistance"))
        })
        .map(|mut booster| {
            // Convert resistances to resonances
            booster.exp_res_bonus = 1.0 - booster.exp_res_bonus;
            booster.kin_res_bonus = 1.0 - booster.kin_res_bonus;
            booster.therm_res_bonus = 1.0 - booster.therm_res_bonus;
            booster
        })
        .collect();

    if boosters.is_empty() {
        println!("Invalid booster rating: {}", test.booster_rating);
        std::process::exit(1);
    }

    let ship_generators: Vec<ShieldGenerator> = generators
        .iter()
        .filter(|sh| sh.ship == test.ship)
        .cloned()
        .collect();

    if ship_generators.is_empty() {
        println!("Unknown ship: {}", test.ship);
        println!("Known Ships:");
        let mut ships = generators
            .into_iter()
            .map(|gen| gen.ship)
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<String>>();
        ships.sort_unstable();

        for ship in ships {
            println!(" * {}", ship);
        }

        std::process::exit(1);
    }

    let generators = ship_generators;

    let min_gen = generators.iter().map(|sh| sh.class).min().unwrap();
    let max_gen = generators.iter().map(|sh| sh.class).max().unwrap();

    let shield_class = test.shield_class.unwrap_or(max_gen);

    if shield_class < min_gen || shield_class > max_gen {
        println!("Invalid shield class: {}", shield_class);
        println!("Valid classes: {} - {}", min_gen, max_gen);
        std::process::exit(1);
    }

    let total_generators = generators.len();

    let generators: Vec<ShieldGenerator> = generators
        .into_iter()
        .filter(|shield| shield.class == shield_class)
        .filter(|shield| !(test.disable_prismatic && shield.kind == "Prismatic"))
        .collect();

    // Filter the booster list using Jamie van den Berge's algorithm:
    //
    // Take each pair of booster, plot their values in a kdtree (effectively
    // modelling a 4-dimensional booster-space), and use that to find pairs which
    // will always be beaten by other pairs on all dimensions.
    //
    // The combinations algorithm then only returns results to test which consist
    // of these pairs.
    let pairs: Vec<Vec<&ShieldBooster>> =
        boosters.iter().combinations_with_replacement(2).collect();
    let total_pairs = pairs.len();

    let pair_metrics: Vec<_> = pairs
        .iter()
        .enumerate()
        .map(|(id, pair)| {
            let exp_res = pair[0].exp_res_bonus * pair[1].exp_res_bonus;
            let kin_res = pair[0].kin_res_bonus * pair[1].kin_res_bonus;
            let therm_res = pair[0].therm_res_bonus * pair[1].therm_res_bonus;
            let shield_strength_bonus =
                pair[0].shield_strength_bonus + pair[1].shield_strength_bonus;
            vec![
                -exp_res,
                -therm_res,
                -kin_res,
                shield_strength_bonus,
                id as f64,
            ]
        })
        .collect();

    let mut tmp_metrics = pair_metrics.clone();
    let tree = kdtree::KDTreeNode::from_points(&mut tmp_metrics[..]).unwrap();

    let filtered_pairs: Vec<(ShieldBooster, ShieldBooster)> = pairs
        .into_iter()
        .zip(pair_metrics.iter())
        .filter(|(_, item)| test.disable_filter || !tree.dominates(&item[..]))
        .map(|(p, _)| (p[0].clone(), p[1].clone()))
        .collect();

    println!();
    println!("---- SEARCH SETUP ----");
    println!(
        "{:>23}: {} of {}",
        "Candidate Shields",
        generators.len(),
        total_generators
    );
    println!(
        "{:>23}: {} of {}",
        "Candidate Boosters",
        boosters.len(),
        total_boosters
    );
    println!(
        "{:>23}: {} of {}",
        "Candidate Booster Pairs",
        filtered_pairs.len(),
        total_pairs
    );

    let mut best_result: Option<TestResult> = None;

    let mut loadouts = 0;
    let start = std::time::Instant::now();

    combinations::unique_selections_from_pairs(
        &boosters[..],
        &filtered_pairs[..],
        test.shield_booster_count.min(8),
        0,
        |booster_loadout| {
            let booster_stat = calculate_booster_stats(&booster_loadout[..]);
            for shield in generators.iter() {
                loadouts += 1;
                let mut stats = calculate_loadout_stats(&shield, &booster_stat);
                // These increase regen time (according to coriolis), and do not stack with boosters
                stats.hit_points += test.reinforced_mj;

                if test
                    .regen_time_limit
                    .map(|limit| calculate_regen_time(&stats) > limit)
                    .unwrap_or(false)
                {
                    continue;
                }

                let actual_dps = calculate_actual_dps(&test, &stats);
                let survival_time = (stats.hit_points + test.shield_cell_mj) / actual_dps;

                let better = best_result
                    .as_ref()
                    .map(|r| {
                        if actual_dps < 0.0 {
                            // Regen exceeds effective DPS, favour better regen,
                            // followed by the highest hitpoints.
                            actual_dps < r.actual_dps
                                || ((actual_dps - r.actual_dps).abs() < std::f64::EPSILON
                                    && stats.hit_points > r.stats.hit_points)
                        } else {
                            r.actual_dps > 0.0 && survival_time > r.survival_time
                        }
                    })
                    .unwrap_or(true);

                if better {
                    best_result = Some(TestResult {
                        actual_dps,
                        survival_time,
                        shield: shield.clone(),
                        boosters: booster_loadout.iter().cloned().cloned().collect(),
                        stats,
                    });
                }
            }
        },
    );

    println!("{:>23}: {:.2?}", "Combinations", loadouts);
    println!("{:>23}: {:.2?}", "Search Time", start.elapsed());

    println!();
    println!("---- TEST SETUP ----");
    println!();
    println!("{:>21}: {}", "Ship Type", test.ship);
    println!("{:>21}: {}", "Shield Class", shield_class);
    println!("{:>21}: {}", "Shield Boosters", test.shield_booster_count);
    println!("{:>21}: {:.1} Mj", "Shield Cell Bank", test.shield_cell_mj);
    println!(
        "{:>21}: {:.1} Mj",
        "Guardian Shield Reinf", test.reinforced_mj
    );
    let limit = if let Some(limit) = test.regen_time_limit {
        format!("{:.1}s", limit)
    } else {
        "no".to_owned()
    };
    println!(
        "{:>21}: {}",
        "Prismatic Shields",
        if test.disable_prismatic { "no" } else { "yes" }
    );
    println!("{:>21}: {}", "Regen Time Limit", limit);
    println!("{:>21}: {}", "Explosive DPS", test.explosive_dps);
    println!("{:>21}: {}", "Kinetic DPS", test.kinetic_dps);
    println!("{:>21}: {}", "Thermal DPS", test.thermal_dps);
    println!("{:>21}: {}", "Absolute DPS", test.absolute_dps);
    println!(
        "{:>21}: {:.1}%",
        "Damage Effectiveness",
        test.damage_effectiveness * 100.0
    );
    println!();
    println!("---- TEST RESULTS ----");
    println!();

    match best_result {
        None => {
            println!("Nothing useful to report.");
        }
        Some(res) => {
            println!(
                "{:>16}: {}",
                "Survival Time",
                if res.survival_time < 0.0 {
                    "∞".to_string()
                } else {
                    format!("{:.1} s", res.survival_time)
                }
            );

            println!("{:>16}: {:.2} Mj/s", "Drain Rate", res.actual_dps);

            println!(
                "{:>16}: {} - {} - {}",
                "Shield Generator",
                res.shield.kind,
                res.shield.engineering,
                res.shield.experimental
            );

            let mut s = "Shield Booster".to_owned();
            for (i, booster) in res.boosters.iter().enumerate() {
                println!(
                    "{:>14} {}: {} - {}",
                    s,
                    i + 1,
                    booster.engineering,
                    booster.experimental
                );
                s.clear();
            }

            println!();
            println!("{:>20}: {:.0} Mj", "Shield Hitpoints", res.stats.hit_points);
            println!(
                "{:>20}: {:.1} Mj/s ({:.1}s from 50%)",
                "Shield Regen Rate",
                res.stats.regen_rate,
                calculate_regen_time(&res.stats)
            );
            println!(
                "{:>20}: {:+.1}% ({:.0} Mj)",
                "Explosive Resistance",
                (1.0 - res.stats.exp_res) * 100.0,
                res.stats.hit_points / res.stats.exp_res
            );
            println!(
                "{:>20}: {:+.1}% ({:.0} Mj)",
                "Kinetic Resistance",
                (1.0 - res.stats.kin_res) * 100.0,
                res.stats.hit_points / res.stats.kin_res
            );
            println!(
                "{:>20}: {:+.1}% ({:.0} Mj)",
                "Thermal Resistance",
                (1.0 - res.stats.therm_res) * 100.0,
                res.stats.hit_points / res.stats.therm_res
            );
        }
    }

    Ok(())
}
