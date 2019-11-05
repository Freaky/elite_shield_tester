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
elite-shield-tester 0.3.0
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
    -s, --shield-booster-count <shield-booster-count>    Number of shield boosters to fit [default: 1]
        --shield-csv <shield-csv>                        Override default shield list
    -t, --thermal-dps <thermal-dps>                      Thermal damage per second [default: 0]
```

It includes a built-in database of shields and boosters from D2EA.

For example, 20 absolute DPS, 60 thermal DPS, 20% hit rate, and 4 shield boosters:

```
> elite_shield_tester -a 20 -t 60 -d 0.2 -s 4
Loaded 45 shields and 25 boosters
Elite Shield Tester Rust Edition v0.3.0
Tested 113805 loadouts in 3.03ms

Attacker:
Explosive DPS: 0
  Kinetic DPS: 0
  Thermal DPS: 60
 Absolute DPS: 20
Effectiveness: 20.0%

Survival Time: 320.3 s
Shield Generator: Bi-Weave - Thermal Resistance - Fast Charge
Shield Boosters:
 * Thermal Resistance - Super Capacitors
 * Thermal Resistance - Super Capacitors
 * Heavy Duty - Super Capacitors
 * Heavy Duty - Super Capacitors

Shield Hitpoints: 1469 Mj
Shield Regen: 5.1 Mj/s
Explosive Resistance: 40.6% (2472 Mj)
  Kinetic Resistance: 14.4% (1717 Mj)
  Thermal Resistance: 61.1% (3778 Mj)
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
