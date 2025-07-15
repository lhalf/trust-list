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
| anyhow      | 362544609 | 23           | 20676                | 100      | 05/10/2019 | 14/04/2025 | https://github.com/dtolnay/anyhow           |
| chrono      | 316490120 | 30+          | 14785                | 91       | 20/11/2014 | 29/04/2025 | https://github.com/chronotope/chrono        |
| clap        | 455668951 | 30+          | 22736                | 433      | 01/03/2015 | 09/06/2025 | https://github.com/clap-rs/clap             |
| field_names | 534190    | 1            | 2                    | 3        | 08/01/2021 | 04/01/2022 | https://github.com/TedDriggs/field_names    |
| itertools   | 538150803 | 30+          | 6517                 | 130      | 21/11/2014 | 31/12/2024 | https://github.com/rust-itertools/itertools |
| pbr         | 2573309   | 26           | 102                  | 24       | 14/10/2015 | 08/02/2023 | https://github.com/a8m/pb                   |
| reqwest     | 236786443 | 30+          | 12502                | 113      | 16/10/2016 | 01/07/2025 | https://github.com/seanmonstar/reqwest      |
| serde       | 572302147 | 30+          | 52086                | 306      | 05/12/2014 | 09/03/2025 | https://github.com/serde-rs/serde           |
| serde_json  | 498510973 | 30+          | 36117                | 172      | 07/08/2015 | 03/03/2025 | https://github.com/serde-rs/json            |

## Compliance

Restricted to one request per second as per: [crates.io data access policy](https://crates.io/data-access#api)
