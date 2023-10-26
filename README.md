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
### Per update
|        Q       | depth=25 | depth=250 | depth=500 |
|:-------------- | :--:     | :--:      | :--:
|       0.5      | 41ns     | 171ns     | 254ns
|       0.6      | 52ns     | 180ns     | 268ns
|       0.7      | 75ns     | 196ns     | 277ns
|       0.8      | 122ns    | 233ns     | 291ns
|       0.9      | 243ns    | 354ns     | 359ns
|       0.95     | 444ns    | 592ns     | 541ns
|       0.99     | 1490s    | 1825ns    | 1590ns

Measured with [binance_robot](src/app/binance_robot.rs) on Apple M1 Pro.

## TODO
- PriceMap pollution detection and rebuild