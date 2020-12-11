# advent-2020

[![GitHub Actions Badge](https://github.com/utter-step/advent-2020/workflows/CI/badge.svg)](https://github.com/utter-step/advent-2020/actions?query=workflow%3ACI)

Rusty Advent of Code solutions — 2020

```console
utterstep@utterstep-nix:~/my/advent-2020$ head /proc/cpuinfo
processor   : 0
vendor_id   : GenuineIntel
cpu family  : 6
model       : 158
model name  : Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz
stepping    : 10
microcod    : 0xde
cpu MHz     : 800.039
cache size  : 9216 KB
physical id : 0

utterstep@utterstep-nix:~/my/advent-2020$ hyperfine --warmup 50 ./target/release/run-all
Benchmark #1: ./target/release/run-all
  Time (mean ± σ):       2.7 ms ±   0.0 ms    [User: 2.4 ms, System: 0.3 ms]
  Range (min … max):     2.6 ms …   3.0 ms    984 runs

  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
```
