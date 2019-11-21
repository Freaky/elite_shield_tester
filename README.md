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
elite_shield_tester 0.4.4
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
        --shield-class <shield-class>                    Shield class (default: maximum possible)
        --shield-csv <shield-csv>                        Override default shield list
        --ship <ship>                                    Ship name [default: Anaconda]
    -t, --thermal-dps <thermal-dps>                      Thermal damage per second [default: 0]

```

It includes a built-in database of ships, shields, and shield boosters.

## Example

Imperial Cutter with class 7 shields, 30 kinetic DPS, 60 thermal DPS, 65% hit
rate, 6 shield boosters &mdash; what's the best shield that still regenerates
within 4 minutes?

```
elite_shield_tester --ship 'Imperial Cutter' --shield-class 7 \
                    -k 30 -t 40 -d 0.6 -s 6 --regen-time-limit 240
Elite Shield Tester Rust Edition v0.4.4

---- SEARCH SETUP ----
      Candidate Shields: 45 of 135
     Candidate Boosters: 20 of 25
Candidate Booster Pairs: 186 of 210
           Combinations: 3324600
            Search Time: 47.64ms

---- TEST SETUP ----

            Ship Type: Imperial Cutter
         Shield Class: 7
      Shield Boosters: 6
     Shield Cell Bank: 0.0 Mj
Guardian Shield Reinf: 0.0 Mj
    Prismatic Shields: yes
     Regen Time Limit: 240.0s
        Explosive DPS: 0
          Kinetic DPS: 30
          Thermal DPS: 40
         Absolute DPS: 0
 Damage Effectiveness: 60.0%

---- TEST RESULTS ----

   Survival Time: 134.4 s
      Drain Rate: 17.70 Mj/s
Shield Generator: Bi-Weave - Reinforced - Fast Charge
Shield Booster 1: Thermal Resistance - Thermo Block
               2: Heavy Duty - Super Capacitors
               3: Heavy Duty - Super Capacitors
               4: Resistance Augmented - Thermo Block
               5: Resistance Augmented - Thermo Block
               6: Resistance Augmented - Super Capacitors

    Shield Hitpoints: 2379 Mj
   Shield Regen Rate: 5.1 Mj/s (235.1s from 50%)
Explosive Resistance: +71.8% (8435 Mj)
  Kinetic Resistance: +66.2% (7030 Mj)
  Thermal Resistance: +43.2% (4189 Mj)
```

## Credits

This tool is based on a [PowerShell script] originally by YouTuber [Down To Earth Astronomy],
and demonstrated in [this video].

It now uses an algorithm developed by [Jamie "Entity" van den Berge], which offers
amazing speedups from the naive approach.  Use `--disable-filter` to bypass this.
Please report if this changes any results.

The `Shield.csv` file is generated using a script utilising [Thurion's Python version].

All data is ultimately derived from [Coriolis].


[Down To Earth Astronomy]: https://www.youtube.com/channel/UCg3QI9rHzPgvR7KTKSCtPHg
[PowerShell script]: https://github.com/DownToEarthAstronomy/D2EA_Shield_tester
[this video]: https://www.youtube.com/watch?v=87DMWz8IeEE
[Elite Dangerous]: https://www.elitedangerous.com/
[website]: https://hur.st/shieldtester/
[Rust]: https://www.rust-lang.org/
[Jamie "Entity" van den Berge]: https://github.com/ntt
[Thurion's Python version]: https://github.com/Thurion/shield_tester
[Coriolis]: https://coriolis.io/
