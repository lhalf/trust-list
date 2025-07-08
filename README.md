# trust-list

[![crates.io](https://img.shields.io/crates/v/trust-list)](https://crates.io/crates/trust-list)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/trust-list-rs/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

Command line tool for generating a trust list from dependencies.

## Install

```bash
cargo install trust-list
```

## Usage

```
$ trust-list --help
Command line tool for generating dependency information tables in markdown

Usage: 

Options:
  -o, --output-file <OUTPUT_FILE>  The output filename, appended with .md [default: trust-list]
  -r, --recreate                   Recreate table [default: appends new dependencies]
  -D, --depth <DEPTH>              The depth of dependencies to collect information on [default: all sub dependencies]
  -d, --dev                        Include dev dependencies [default: excluded]
  -b, --build                      Include build dependencies [default: excluded]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Example

```bash
trust-list --depth 1
```

| name        | downloads | contributors | reverse_dependencies | versions | created_at | updated_at | repository                                  |
|-------------|-----------|--------------|----------------------|----------|------------|------------|---------------------------------------------|
| anyhow      | 362002016 | 23           | 20659                | 100      | 05/10/2019 | 14/04/2025 | https://github.com/dtolnay/anyhow           |
| chrono      | 316018023 | 30+          | 14773                | 91       | 20/11/2014 | 29/04/2025 | https://github.com/chronotope/chrono        |
| clap        | 454984340 | 30+          | 22719                | 433      | 01/03/2015 | 09/06/2025 | https://github.com/clap-rs/clap             |
| field_names | 533994    | 1            | 2                    | 3        | 08/01/2021 | 04/01/2022 | https://github.com/TedDriggs/field_names    |
| itertools   | 537009430 | 30+          | 6515                 | 130      | 21/11/2014 | 31/12/2024 | https://github.com/rust-itertools/itertools |
| reqwest     | 236309913 | 30+          | 12495                | 113      | 16/10/2016 | 01/07/2025 | https://github.com/seanmonstar/reqwest      |
| serde       | 426837886 | 30+          | 43946                | 304      | 05/12/2014 | 03/03/2025 | https://github.com/serde-rs/serde           |
| serde_json  | 497883329 | 30+          | 36095                | 172      | 07/08/2015 | 03/03/2025 | https://github.com/serde-rs/json            |

## Compliance

Restricted to one request per second as per: [crates.io data access policy](https://crates.io/data-access#api)
