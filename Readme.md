# Advent of Code

My solutions, written in Rust. I've tried to write idiomatic, readable code. I generally use iterators where I can.

I attempt to make my code fast, but if it's a choice between fast and readable, I choose readable. No unsafe code or SIMD here.

## Setup

1. Clone this repo - `gh repo clone nindalf/advent` or `git clone git@github.com:nindalf/advent.git`.
2. Install [Rust](https://www.rust-lang.org/learn/get-started) or update - `rustup update`.
3. Install [just](https://just.systems), the command runner.
4. Install [aocgen](https://github.com/nindalf/aocgen), which fetches the problem input and creates the empty files.

## How to use

Run `just` for all the available commands.

By default `just` will run these for the latest year, set by the env variable `AOC_YEAR`.

```
just fetch 15    # fetches the 15th day's problem and input.

just test 15 1_t # runs day15::tests::part1_test
just test 15 2   # runs day15::tests::part2_test _and_ day15::tests:part2
just test 15     # runs all 4 tests for day 15
just test        # runs all tests for all days of AOC_YEAR

just bench 15    # benchmarks day 15 parts 1 and 2
just bench       # runs all benchmarks for all days of AOC_YEAR
```

### Overriding `AOC_YEAR`

If `AOC_YEAR` is not set, it picks up the default from the `justfile`. To run the commands for a different year, you can choose one of these options:

- Set it permanently
  - Set the env variable - `export AOC_YEAR=2023`
  - Change the default in the `justfile` - `AOC_YEAR := env_var_or_default("AOC_YEAR", "2023")`
- Set it for one invocation
  - `AOC_YEAR=2023 just test` OR
  - `just --set AOC_YEAR 2023 test`
