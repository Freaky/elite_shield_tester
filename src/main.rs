use std::path::PathBuf;
use std::error::Error;
use std::path::Path;
use std::io::Read;

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
#[structopt(name = "elite_shield_tester", about = "Rust port of Down To Earth Astronomy's script")]
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
    stats: LoadoutStat
}

fn load_csv<P, T>(path: P) -> Result<Vec<T>, csv::Error>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let mut reader = csv::Reader::from_path(path)?;
    let mut ret = vec![];
    for record in reader.deserialize() {
        let record: T = record?;
        ret.push(record);
    }
    Ok(ret)
}

fn parse_csv<T, R>(s: R) -> Result<Vec<T>, csv::Error>
where
    R: Read,
    T: DeserializeOwned
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
    let mut hit_point_bonus = 0.0;

    for booster in boosters.iter() {
        exp_modifier *= 1.0 - booster.exp_res_bonus;
        kin_modifier *= 1.0 - booster.kin_res_bonus;
        therm_modifier *= 1.0 - booster.therm_res_bonus;

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

    let exp_res = 1.0 - ((1.0 - shield.exp_res) * exp_modifier);
    let kin_res = 1.0 - ((1.0 - shield.kin_res) * kin_modifier);
    let therm_res = 1.0 - ((1.0 - shield.therm_res) * therm_modifier);

    let hit_points = (1.0 + hit_point_bonus) * shield.shield_strength;

    LoadoutStat {
        hit_points,
        exp_res,
        kin_res,
        therm_res,
        regen_rate: shield.regen_rate,
    }
}

fn calculate_survival_time(test: &TestConfig, loadout: &LoadoutStat) -> f32 {
    let actual_dps = test.damage_effectiveness
        * (test.explosive_dps * (1.0 - loadout.exp_res)
            + test.kinetic_dps * (1.0 - loadout.kin_res)
            + test.thermal_dps * (1.0 - loadout.therm_res)
            + test.absolute_dps)
        - loadout.regen_rate * (1.0 - test.damage_effectiveness);

    loadout.hit_points / actual_dps
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = TestConfig::from_args();

    let mut generators: Vec<ShieldGenerator> = parse_csv(&include_bytes!("../data/ShieldGeneratorVariants.csv")[..])?;
    let mut boosters: Vec<ShieldBooster> = parse_csv(&include_bytes!("../data/ShieldBoosterVariants.csv")[..])?;

    if let Some(ref path) = test.shield_csv {
        println!("Custom Shield CSV: {}", path.display());
        generators = load_csv(path)?;
    }

    if let Some(ref path) = test.booster_csv {
        println!("Custom Booster CSV: {}", path.display());
        boosters = load_csv(path)?;
    }

    let generators = generators;
    let boosters = boosters;

    println!("Loaded {} shields and {} boosters", generators.len(), boosters.len());

    // let test = TestConfig {
    //     shield_booster_count: 7,
    //     explosive_dps: 14.0,
    //     kinetic_dps: 18.0,
    //     thermal_dps: 110.0,
    //     absolute_dps: 0.0,
    //     damage_effectiveness: 0.25,
    // };

    let mut best_survival_time = 0.0;
    let mut best_result: Option<TestResult> = None;

    let mut loadouts = 0;
    let start = std::time::Instant::now();

    for booster_loadout in boosters
        .iter()
        .combinations_with_replacement(test.shield_booster_count as usize)
    {
        for shield in generators.iter() {
            loadouts += 1;
            let stats = calculate_loadout_stats(&shield, &booster_loadout[..]);
            let survival_time = calculate_survival_time(&test, &stats);

            if survival_time > best_survival_time {
                best_survival_time = survival_time;
                best_result = Some(
                    TestResult {
                        shield: shield.clone(),
                        boosters: booster_loadout.iter().cloned().cloned().collect(),
                        stats
                    }
                );
            }
        }
    }

    println!("Tested {} loadouts in {:.2?}", loadouts, start.elapsed());

    println!();

    match best_result {
        None => {
            println!("Nothing useful to report.");
        },
        Some(res) => {
            println!("Survival Time [s]: [{:.1}]", best_survival_time);
            println!(
                "Shield Generator: [{}] - [{}] - [{}]",
                res.shield.kind, res.shield.engineering, res.shield.experimental
            );
            println!("Shield Boosters:");

            for booster in res.boosters {
                println!("[{}] - [{}]", booster.engineering, booster.experimental);
            }

            println!();
            println!("Shield Hitpoints: [{:.1}]", res.stats.hit_points);
            println!("Shield Regen: [{:.1} hp/s]", res.stats.regen_rate);
            println!("Explosive Resistance: [{:.1}%]", res.stats.exp_res * 100.0);
            println!("  Kinetic Resistance: [{:.1}%]", res.stats.kin_res * 100.0);
            println!("  Thermal Resistance: [{:.1}%]", res.stats.therm_res * 100.0);
        }
    }

    Ok(())
}
