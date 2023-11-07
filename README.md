<p align="center">
  <img src="assets/logo.png" alt="Lobotomy" width=500>
</p>

<p align="center">
    <i>Where Performance Meets Peace of Mind</i>
</p>

# Lobotomy
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)



Lobotomy is a Limit Order Book (and more) implementation in Rust, designed for safety and performance.

## Latency
### L2 from L3 deltas, per update
| Q              | depth=16384 |
| :------------- | :-----: |
| 0.5            |   42ns  |
| 0.6            |   42ns  |
| 0.7            |   83ns  |
| 0.8            |   125ns |
| 0.9            |   208ns |
| 0.95           |   292ns |
| 0.99           |   792ns |


Measured with [nasdaq_robot](src/app/nasdaq_robot.rs) on Apple M1 Pro.

### L2 from L2 upserts, per update
| Q              | depth=16384 |
| :------------- | :-----: |
| 0.5            |   804ns  |
| 0.6            |   845ns  |
| 0.7            |   907s  |
| 0.8            |   1024ns  |
| 0.9            |   1259ns  |
| 0.95           |   1569ns  |
| 0.99           |   27085ns |


Measured with [binance_robot](src/app/binance_robot.rs) on Apple M1 Pro.