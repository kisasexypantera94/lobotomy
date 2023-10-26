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
### Level-based, per update
| Q              | depth=5 | depth=25 | depth=250 | depth=500 |
| :------------- | :-----: | :------: | :-------: | :-------: |
| 0.5            |   18ns  |   29ns   |   163ns   |   254ns   |
| 0.6            |   20ns  |   31ns   |   168ns   |   268ns   |
| 0.7            |   26ns  |   34ns   |   176ns   |   277ns   |
| 0.8            |   34ns  |   40ns   |   191ns   |   291ns   |
| 0.9            |   58ns  |   53ns   |   230ns   |   359ns   |
| 0.95           |   93ns  |   77ns   |   286ns   |   541ns   |
| 0.99           |   445ns |   183ns  |   605ns   |   1590ns  |


Measured with [binance_robot](src/app/binance_robot.rs) on Apple M1 Pro.

## TODO
- PriceMap pollution detection and rebuild