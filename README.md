# trust-list-rs

[![crates.io](https://img.shields.io/crates/v/reqwest.svg)](https://crates.io/crates/trust-list)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

Command line tool for automatically generating a trust list of your dependencies.

## Install

`cargo install trust-list`

## Usage

`trust-list --help`

## Example

`trust-list --depth 1`

| crate      | downloads | contributors | reverse dependencies | releases | created    | last updated | link                                        |
|------------|-----------|--------------|----------------------|----------|------------|--------------|---------------------------------------------|
| anyhow     | 259628945 | 23           | 16853                | 96       | 05/10/2019 | 03/12/2024   | https://github.com/dtolnay/anyhow           |
| chrono     | 230302961 | 30+          | 12441                | 89       | 20/11/2014 | 09/12/2024   | https://github.com/chronotope/chrono        |
| clap       | 327312749 | 30+          | 19088                | 416      | 01/03/2015 | 05/12/2024   | https://github.com/clap-rs/clap             |
| itertools  | 336233196 | 30+          | 5662                 | 129      | 21/11/2014 | 16/05/2024   | https://github.com/rust-itertools/itertools |
| log        | 358195242 | 30+          | 19137                | 52       | 13/12/2014 | 28/06/2024   | https://github.com/rust-lang/log            |
| pbr        | 2135671   | 26           | 97                   | 24       | 14/10/2015 | 08/02/2023   | https://github.com/a8m/pb                   |
| reqwest    | 159413648 | 30+          | 10173                | 100      | 16/10/2016 | 28/10/2024   | https://github.com/seanmonstar/reqwest      |
| serde      | 423059152 | 30+          | 43603                | 303      | 05/12/2014 | 11/12/2024   | https://github.com/serde-rs/serde           |
| serde_json | 367280268 | 30+          | 29690                | 165      | 07/08/2015 | 17/11/2024   | https://github.com/serde-rs/json            |

Restricted to one request per second as per: [crates.io data access policy](https://crates.io/data-access#api)
