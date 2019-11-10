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
elite-shield-tester 0.4.1
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
        --regen-time-limit <regen-time-limit>
            Disregard shields that take longer than this many seconds to regenerate from 50%

        --reinforced-mj <reinforced-mj>                  Mj provided by Guardian Shield Reinforcements [default: 0]
    -s, --shield-booster-count <shield-booster-count>    Number of shield boosters to fit [default: 1]
        --shield-cell-mj <shield-cell-mj>                Mj available via Shield Cell Banks [default: 0]
        --shield-csv <shield-csv>                        Override default shield list
    -t, --thermal-dps <thermal-dps>                      Thermal damage per second [default: 0]
```

It includes a built-in database of shields and boosters from D2EA.  Currently
all boosters are fully-engineered A-rated and results are based on an Anaconda
with class 7 shields.  Generalising to all ships, shield sizes, booster classes,
and engineering levels is on the todo.

## Example

30 kinetic DPS, 60 thermal DPS, 65% hit rate, 6 shield boosters &mdash; what's
the best shield that still regenerates within 4 minutes?

```
-% elite_shield_tester -k 30 -t 40 -d 0.65 -s 6 --regen-time-limit 240
Elite Shield Tester Rust Edition v0.4.1

---- SEARCH SETUP ----
      Candidate Shields: 45 of 45
     Candidate Boosters: 20 of 25
Candidate Booster Pairs: 186 of 210
           Combinations: 3324600
            Search Time: 51.45ms

---- TEST SETUP ----

      Shield Boosters: 6
     Shield Cell Bank: 0.0 Mj
Guardian Shield Reinf: 0.0 Mj
    Prismatic Shields: yes
     Regen Time Limit: 240.0s
        Explosive DPS: 0
          Kinetic DPS: 30
          Thermal DPS: 40
         Absolute DPS: 0
 Damage Effectiveness: 65.0%

---- TEST RESULTS ----

Survival Time   : 116.4 s
Shield Generator: Bi-Weave - Reinforced - Fast Charge
Shield Booster 1: Heavy Duty - Thermo Block
               2: Heavy Duty - Thermo Block
               3: Heavy Duty - Super Capacitors
               4: Resistance Augmented - Thermo Block
               5: Resistance Augmented - Thermo Block
               6: Resistance Augmented - Thermo Block

    Shield Hitpoints: 2421 Mj
   Shield Regen Rate: 5.1 Mj/s (237.3s from 50%)
Explosive Resistance: +72.8% (8902 Mj)
  Kinetic Resistance: +67.4% (7419 Mj)
  Thermal Resistance: +37.6% (3879 Mj)
```

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
