# `ambient-weather-progenitor`

A client for ambient-weather's REST api generated using [progenitor](https://github.com/oxidecomputer/progenitor).

[`ambient-weather.json`](./ambient-weather.json) has been generated from <https://github.com/ambient-weather/api-docs/blob/d68ffa5ffda4975391b0ef9ca7833c1ce567cd1d/apiary.apib> using [apib2swagger](https://github.com/kminami/apib2swagger), with the operation ids renamed to something that should work with most codegen tools, and `date-time` format added where appropriate.

To run the code generator, do the following:

```bash
$ cargo run -p codegen
```
