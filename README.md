# Elite Dangerous Shield Tester

This is a tool for determining optimal shield loadouts for ships in the space
simulation game [Elite Dangerous].

You feed it a proposed attacker with a given set of damage types and effectiveness,
and it determines the best combination of shield and booster engineering to maximise
survival time against that opponent.

This tool can also be queried on its [website].


## Usage

This is a command-line application, run it with --help to get usage:

```
elite-shield-tester 0.4.0
Elite Dangerous Shield Optimiser

USAGE:
    elite_shield_tester [FLAGS] [OPTIONS]

FLAGS:
        --disable-filter        Disable pre-filtering (debugging)
        --disable-prismatic     Filter out prismatic shields
        --force-experimental    Require experimental effects
    -h, --help                  Prints help information
    -V, --version               Prints version information

OPTIONS:
    -a, --absolute-dps <absolute-dps>                    Absolute damage per second [default: 0]
        --booster-csv <booster-csv>                      Override default booster list
    -d, --damage-effectiveness <damage-effectiveness>    Attacker shot success ratio, 0-1 [default: 0.5]
    -e, --explosive-dps <explosive-dps>                  Explosive damage per second [default: 0]
    -k, --kinetic-dps <kinetic-dps>                      Kinetic damage per second [default: 0]
        --reinforced-mj <reinforced-mj>                  Mj provided by Guardian Shield Reinforcements [default: 0]
    -s, --shield-booster-count <shield-booster-count>    Number of shield boosters to fit [default: 1]
        --shield-cell-mj <shield-cell-mj>                Mj available via Shield Cell Banks [default: 0]
        --shield-csv <shield-csv>                        Override default shield list
    -t, --thermal-dps <thermal-dps>                      Thermal damage per second [default: 0]
```

It includes a built-in database of shields and boosters from D2EA.

For example, 20 absolute DPS, 60 thermal DPS, 65% hit rate, and 4 shield boosters:

```
> elite_shield_tester -a 20 -t 60 -d 0.65 -s 4
Elite Shield Tester Rust Edition v0.4.0
Loaded 45 shields and 15 boosters
Tested 137700 loadouts in 1.82ms

Test Setup:
Shield Boosters: 4
Shield Cell Bank Pool: 0.0 Mj
Guardian Shield Reinforcement: 0.0 Mj
Explosive DPS: 0
  Kinetic DPS: 0
  Thermal DPS: 60
 Absolute DPS: 20
Effectiveness: 65.0%

Survival Time: 83.1 s
Shield Generator: Prismatic - Thermal Resistance - Hi-Cap
Shield Boosters:
 * Thermal Resistance - Thermo Block
 * Heavy Duty - Super Capacitors
 * Heavy Duty - Super Capacitors
 * Heavy Duty - Super Capacitors

Shield Hitpoints: 2526 Mj
Shield Regen: 1.1 Mj/s
Explosive Resistance: 44.8% (4578 Mj)
  Kinetic Resistance: 20.5% (3179 Mj)
  Thermal Resistance: 54.4% (5546 Mj)
````


## Credits

This tool is based on a [PowerShell script] originally by YouTuber [Down To Earth Astronomy],
and demonstrated in [this video].

It now uses an algorithm developed by [Jamie "Entity" van den Berge], which offers
amazing speedups from the naive approach.  Use `--disable-filter` to bypass this.
Please report if this changes any results.


[Down To Earth Astronomy]: https://www.youtube.com/channel/UCg3QI9rHzPgvR7KTKSCtPHg
[PowerShell script]: https://github.com/DownToEarthAstronomy/D2EA_Shield_tester
[this video]: https://www.youtube.com/watch?v=87DMWz8IeEE
[Elite Dangerous]: https://www.elitedangerous.com/
[website]: https://hur.st/shieldtester/
[Rust]: https://www.rust-lang.org/
[Jamie "Entity" van den Berge]: https://github.com/ntt
