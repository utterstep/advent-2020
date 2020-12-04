# advent-2020

[![GitHub Actions Badge](https://github.com/utter-step/advent-2020/workflows/CI/badge.svg)](https://github.com/utter-step/advent-2020/actions?query=workflow%3ACI)

Rusty Advent of Code solutions — 2020

```(bash)
utterstep@utterstep-nix:~/my/advent-2020$ hyperfine --warmup 50 ./target/release/run-all
Benchmark #1: ./target/release/run-all
  Time (mean ± σ):       1.0 ms ±   0.1 ms    [User: 0.9 ms, System: 0.1 ms]
  Range (min … max):     0.8 ms …   2.3 ms    1959 runs

  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
