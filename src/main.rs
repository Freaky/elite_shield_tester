use std::error::Error;
use std::io::Read;
use std::path::PathBuf;

use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ShieldGenerator {
    #[serde(rename = "ID")]
    id: u8,
    #[serde(rename = "Type")]
    kind: String,
    engineering: String,
    experimental: String,
    shield_strength: f32,
    regen_rate: f32,
    exp_res: f32,
    kin_res: f32,
    therm_res: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ShieldBooster {
    #[serde(rename = "ID")]
    id: u8,
    engineering: String,
    experimental: String,
    shield_strength_bonus: f32,
    exp_res_bonus: f32,
    kin_res_bonus: f32,
    therm_res_bonus: f32,
}

#[derive(Debug, Clone)]
struct LoadoutStat {
    hit_points: f32,
    regen_rate: f32,
    exp_res: f32,
    kin_res: f32,
    therm_res: f32,
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "elite_shield_tester",
    about = "Rust port of Down To Earth Astronomy's script"
)]
struct TestConfig {
    /// Number of shield boosters to fit
    #[structopt(short, long, default_value = "1")]
    shield_booster_count: u8,
    /// Explosive damage per second
    #[structopt(short, long, default_value = "0")]
    explosive_dps: f32,
    /// Kinetic damage per second
    #[structopt(short, long, default_value = "0")]
    kinetic_dps: f32,
    /// Thermal damage per second
    #[structopt(short, long, default_value = "0")]
    thermal_dps: f32,
    /// Absolute damage per second
    #[structopt(short, long, default_value = "0")]
    absolute_dps: f32,
    /// Attacker shot success ratio, 0-1
    #[structopt(short, long, default_value = "0.5")]
    damage_effectiveness: f32,
    /// Filter out prismatic shields
    #[structopt(long)]
    disable_prismatic: bool,
    /// Require experimental effects (~5x faster)
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
}

#[derive(Debug, Clone)]
struct TestResult {
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

fn calculate_loadout_stats(shield: &ShieldGenerator, boosters: &[&ShieldBooster]) -> LoadoutStat {
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

    if exp_modifier < 0.7 {
        exp_modifier = 0.7 - (0.7 - exp_modifier) / 2.0;
    }

    if kin_modifier < 0.7 {
        kin_modifier = 0.7 - (0.7 - kin_modifier) / 2.0;
    }

    if therm_modifier < 0.7 {
        therm_modifier = 0.7 - (0.7 - therm_modifier) / 2.0;
    }

    LoadoutStat {
        hit_points: hit_point_bonus * shield.shield_strength,
        exp_res: shield.exp_res * exp_modifier,
        kin_res: shield.kin_res * kin_modifier,
        therm_res: shield.therm_res * therm_modifier,
        regen_rate: shield.regen_rate,
    }
}

fn calculate_survival_time(test: &TestConfig, loadout: &LoadoutStat) -> f32 {
    let actual_dps = test.damage_effectiveness
        * (test.explosive_dps * loadout.exp_res
            + test.kinetic_dps * loadout.kin_res
            + test.thermal_dps * loadout.therm_res
            + test.absolute_dps)
        - loadout.regen_rate * (1.0 - test.damage_effectiveness);

    loadout.hit_points / actual_dps
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = TestConfig::from_args();

    let mut generators: Vec<ShieldGenerator> =
        parse_csv(&include_bytes!("../data/ShieldGeneratorVariants.csv")[..])?;
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

    let boosters: Vec<ShieldBooster> = boosters
        .into_iter()
        .map(|mut booster| {
            // Convert resistances to resonances
            booster.exp_res_bonus = 1.0 - booster.exp_res_bonus;
            booster.kin_res_bonus = 1.0 - booster.kin_res_bonus;
            booster.therm_res_bonus = 1.0 - booster.therm_res_bonus;
            booster
        })
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
        .collect();

    let generators: Vec<ShieldGenerator> = generators
        .into_iter()
        .map(|mut shield| {
            shield.exp_res = 1.0 - shield.exp_res;
            shield.kin_res = 1.0 - shield.kin_res;
            shield.therm_res = 1.0 - shield.therm_res;
            shield
        })
        .filter(|shield| !(test.disable_prismatic && shield.kind == "Prismatic"))
        .collect();

    println!(
        "Loaded {} shields and {} boosters",
        generators.len(),
        boosters.len()
    );

    let mut best_survival_time = 0.0;
    let mut best_result: Option<TestResult> = None;

    let mut loadouts = 0;
    let start = std::time::Instant::now();

    for booster_loadout in boosters
        .iter()
        .combinations_with_replacement(test.shield_booster_count.min(8) as usize)
    {
        for shield in generators.iter() {
            loadouts += 1;
            let stats = calculate_loadout_stats(&shield, &booster_loadout[..]);
            let survival_time = calculate_survival_time(&test, &stats);

            // Negative survival times indicate regen exceeds DPS
            if (survival_time < 0.0
                && (best_survival_time >= 0.0 || survival_time > best_survival_time))
                || (survival_time >= 0.0
                    && best_survival_time >= 0.0
                    && survival_time > best_survival_time)
            {
                best_survival_time = survival_time;
                best_result = Some(TestResult {
                    shield: shield.clone(),
                    boosters: booster_loadout.iter().cloned().cloned().collect(),
                    stats,
                });
            }
        }
    }

    println!("Tested {} loadouts in {:.2?}", loadouts, start.elapsed());

    println!();
    println!("Attacker:");
    println!("Explosive DPS: {}", test.explosive_dps);
    println!("  Kinetic DPS: {}", test.kinetic_dps);
    println!("  Thermal DPS: {}", test.thermal_dps);
    println!(" Absolute DPS: {}", test.absolute_dps);
    println!("Effectiveness: {:.1}%", test.damage_effectiveness * 100.0);
    println!();

    match best_result {
        None => {
            println!("Nothing useful to report.");
        }
        Some(res) => {
            if best_survival_time < 0.0 {
                println!("Survival Time: ∞");
            } else {
                println!("Survival Time: {:.1} s", best_survival_time);
            }
            println!(
                "Shield Generator: {} - {} - {}",
                res.shield.kind, res.shield.engineering, res.shield.experimental
            );
            println!("Shield Boosters:");

            for booster in res.boosters {
                println!(" * {} - {}", booster.engineering, booster.experimental);
            }

            println!();
            println!("Shield Hitpoints: {:.0} Mj", res.stats.hit_points);
            println!("Shield Regen: {:.1} Mj/s", res.stats.regen_rate);
            println!(
                "Explosive Resistance: {:.1}% ({:.0} Mj)",
                (1.0 - res.stats.exp_res) * 100.0,
                res.stats.hit_points / res.stats.exp_res
            );
            println!(
                "  Kinetic Resistance: {:.1}% ({:.0} Mj)",
                (1.0 - res.stats.kin_res) * 100.0,
                res.stats.hit_points / res.stats.kin_res
            );
            println!(
                "  Thermal Resistance: {:.1}% ({:.0} Mj)",
                (1.0 - res.stats.therm_res) * 100.0,
                res.stats.hit_points / res.stats.therm_res
            );
        }
    }

    Ok(())
}
