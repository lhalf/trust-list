# trust-list-rs

[![crates.io](https://img.shields.io/crates/v/trust-list)](https://crates.io/crates/trust-list)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/trust-list-rs/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

Command line tool for generating a trust list from dependencies.

## Install

`cargo install trust-list`

## Usage

`trust-list --help`

## Example

`trust-list --depth 1`

| crate      | downloads | contributors | reverse dependencies | releases | created    | last updated | link                                        |
|------------|-----------|--------------|----------------------|----------|------------|--------------|---------------------------------------------|
| anyhow     | 262160669 | 23           | 17000                | 97       | 05/10/2019 | 22/12/2024   | https://github.com/dtolnay/anyhow           |
| chrono     | 232267527 | 30+          | 12555                | 89       | 20/11/2014 | 09/12/2024   | https://github.com/chronotope/chrono        |
| clap       | 330171079 | 30+          | 19266                | 416      | 01/03/2015 | 05/12/2024   | https://github.com/clap-rs/clap             |
| itertools  | 339764411 | 30+          | 5699                 | 130      | 21/11/2014 | 31/12/2024   | https://github.com/rust-itertools/itertools |
| pbr        | 2147945   | 26           | 97                   | 24       | 14/10/2015 | 08/02/2023   | https://github.com/a8m/pb                   |
| reqwest    | 160886858 | 30+          | 10274                | 103      | 16/10/2016 | 31/12/2024   | https://github.com/seanmonstar/reqwest      |
| serde      | 426837886 | 30+          | 43946                | 304      | 05/12/2014 | 27/12/2024   | https://github.com/serde-rs/serde           |
| serde_json | 370649558 | 30+          | 29948                | 166      | 07/08/2015 | 21/12/2024   | https://github.com/serde-rs/json            |

Restricted to one request per second as per: [crates.io data access policy](https://crates.io/data-access#api)
