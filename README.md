# Elite Dangerous Shield Tester

This is a [Rust] rewrite of Down to Earth Astronomy's [Shield Tester] PowerShell
script, as demonstrated in [this video].

It allows for determining the optimal combinations of shield, shield boosters
and engineering modifications to defend against a given assailant in the space
simulation game [Elite Dangerous].

This version runs a great deal faster, and can also be queried via a [website].


## Usage

This is a command-line application, run it with --help to get usage:

```
elite-shield-tester 0.1.0
Rust port of Down To Earth Astronomy's script

USAGE:
    elite_shield_tester.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

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
> elite_shield_tester.exe -a 20 -t 60 -d 0.2 -s 4
Loaded 45 shields and 20 boosters
Tested 398475 loadouts in 6.15ms

Survival Time [s]: [320.4]
Shield Generator: [Bi-Weave] - [Thermal Resistance] - [Fast Charge]
Shield Boosters:
[Thermal Resistance] - [Super Capacitors]
[Thermal Resistance] - [Super Capacitors]
[Heavy Duty] - [Super Capacitors]
[Heavy Duty] - [Super Capacitors]

Shield Hitpoints: [1469.0]
Shield Regen: [5.1 hp/s]
Explosive Resistance: [39.4%]
  Kinetic Resistance: [12.8%]
  Thermal Resistance: [61.1%]
````

[Shield Tester]: https://github.com/DownToEarthAstronomy/D2EA_Shield_tester
[this video]: https://www.youtube.com/watch?v=87DMWz8IeEE
[Elite Dangerous]: https://www.elitedangerous.com/
[website]: https://hur.st/shieldtester/
[Rust]: https://www.rust-lang.org/
