# trust-list

[![crates.io](https://img.shields.io/crates/v/trust-list)](https://crates.io/crates/trust-list)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/trust-list/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

Command line tool for generating a dependency information table in markdown.

## Install

```bash
cargo install trust-list
```

## Usage

```
$ trust-list --help
Command line tool for generating a dependency information table in markdown

Usage: trust-list [OPTIONS]

Options:
  -o, --output-file <OUTPUT_FILE>  The output filename, appended with .md [default: trust-list]
  -r, --recreate                   Recreate table [default: appends new dependencies]
  -D, --depth <DEPTH>              The depth of dependencies to collect information on [default: all sub dependencies]
  -d, --dev                        Include dev dependencies [default: excluded]
  -b, --build                      Include build dependencies [default: excluded]
  -e, --exclude <EXCLUDE>          Exclude specified workspace [default: all included]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Example

```bash
trust-list --depth 1
```

| name        | downloads | contributors | reverse_dependencies | versions | created_at | updated_at | repository                                  |
|-------------|-----------|--------------|----------------------|----------|------------|------------|---------------------------------------------|
| anyhow      | 455074655 | 24           | 24558                | 102      | 05/10/2019 | 19/09/2025 | https://github.com/dtolnay/anyhow           |
| chrono      | 393631079 | 30+          | 17491                | 92       | 20/11/2014 | 08/09/2025 | https://github.com/chronotope/chrono        |
| clap        | 564552921 | 30+          | 25926                | 444      | 01/03/2015 | 29/10/2025 | https://github.com/clap-rs/clap             |
| field_names | 556548    | 1            | 3                    | 3        | 08/01/2021 | 04/01/2022 | https://github.com/TedDriggs/field_names    |
| itertools   | 701399480 | 30+          | 7038                 | 130      | 21/11/2014 | 31/12/2024 | https://github.com/rust-itertools/itertools |
| pbr         | 2835208   | 26           | 105                  | 24       | 14/10/2015 | 08/02/2023 | https://github.com/a8m/pb                   |
| reqwest     | 307663439 | 30+          | 14612                | 115      | 16/10/2016 | 13/10/2025 | https://github.com/seanmonstar/reqwest      |
| serde       | 701667189 | 30+          | 59544                | 315      | 05/12/2014 | 27/09/2025 | https://github.com/serde-rs/serde           |
| serde_json  | 616227930 | 30+          | 41960                | 177      | 07/08/2015 | 14/09/2025 | https://github.com/serde-rs/json            |

## Compliance

Restricted to one request per second as per: [crates.io data access policy](https://crates.io/data-access#api)
