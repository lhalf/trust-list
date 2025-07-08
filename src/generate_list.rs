use crate::crates_io::get_crate_info;
use crate::file_io::FileIO;
use crate::github::get_contributor_count;
use crate::http_client::GetRequest;
use anyhow::Error;
use std::collections::BTreeSet;

pub fn generate_list(
    crate_names: BTreeSet<String>,
    output_file: impl FileIO,
    http_client: impl GetRequest,
) -> Result<(), Error> {
    let existing_names = parse_existing_crate_names(&output_file.read_to_string()?);
    let missing_names = crate_names.difference(&existing_names);

    for crate_name in missing_names {
        match get_crate_info(&http_client, crate_name) {
            Ok(mut crate_info) => {
                crate_info.contributors =
                    get_contributor_count(&http_client, &crate_info.repository)?;

                output_file.append(crate_info.table_entry().as_bytes())?;

                println!("{crate_name}");
            }
            Err(error) => {
                println!("failed to get info for {crate_name}: {error}");
            }
        }
    }

    Ok(())
}

fn parse_existing_crate_names(contents: &str) -> BTreeSet<String> {
    contents
        .lines()
        .skip(2)
        .filter_map(|line| line.split('|').nth(1))
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::file_io::FileIOSpy;
    use crate::generate_list::generate_list;
    use crate::http_client::GetRequestSpy;
    use std::collections::BTreeSet;

    #[test]
    fn output_file_exists_but_cant_be_read() {
        let crates = BTreeSet::new();
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert_eq!(
            "deliberate test error",
            generate_list(crates, file_io_spy, http_client_spy)
                .unwrap_err()
                .to_string()
        )
    }

    #[test]
    fn no_crates_are_required() {
        let crates = BTreeSet::new();
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        assert!(generate_list(crates, file_io_spy, http_client_spy).is_ok())
    }

    #[test]
    fn single_crate_required_get_crate_info_fails() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        http_client_spy
            .get
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert!(generate_list(crates, file_io_spy, http_client_spy).is_ok())
    }

    #[test]
    fn single_crate_required_get_reverse_dependencies_fails() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        http_client_spy
            .get
            .returns
            .push_back(Ok(include_str!("../tests/data/crate_info.json").to_string()));
        http_client_spy
            .get
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert!(generate_list(crates, file_io_spy, http_client_spy).is_ok())
    }

    // TODO: this should probably not fail
    #[test]
    fn single_crate_required_get_contributor_count_fails() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        http_client_spy
            .get
            .returns
            .push_back(Ok(include_str!("../tests/data/crate_info.json").to_string()));
        http_client_spy.get.returns.push_back(Ok(
            r#"{ "dependencies": [], "versions": [], "meta": { "total": 32 } }"#.to_string(),
        ));
        http_client_spy
            .get
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert_eq!(
            "deliberate test error",
            generate_list(crates, file_io_spy, http_client_spy)
                .unwrap_err()
                .to_string()
        )
    }

    #[test]
    fn single_crate_required_appending_table_fails() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        http_client_spy
            .get
            .returns
            .push_back(Ok(include_str!("../tests/data/crate_info.json").to_string()));
        http_client_spy.get.returns.push_back(Ok(
            r#"{ "dependencies": [], "versions": [], "meta": { "total": 32 } }"#.to_string(),
        ));
        http_client_spy
            .get
            .returns
            .push_back(Ok(r#"[1,2,3,4,5]"#.to_string()));

        file_io_spy
            .append
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert_eq!(
            "deliberate test error",
            generate_list(crates, file_io_spy, http_client_spy)
                .unwrap_err()
                .to_string()
        )
    }

    #[test]
    fn single_crate_required_appends_expected_line_to_table() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        http_client_spy
            .get
            .returns
            .push_back(Ok(include_str!("../tests/data/crate_info.json").to_string()));
        http_client_spy.get.returns.push_back(Ok(
            r#"{ "dependencies": [], "versions": [], "meta": { "total": 32 } }"#.to_string(),
        ));
        http_client_spy
            .get
            .returns
            .push_back(Ok(r#"[1,2,3,4,5]"#.to_string()));

        file_io_spy.append.returns.push_back(Ok(()));

        assert!(generate_list(crates, file_io_spy.clone(), http_client_spy).is_ok());
        assert_eq!(
            vec![
                b"|autospy|1861|5|32|8|15/05/2025|01/07/2025|https://github.com/lhalf/autospy|\n"
                    .to_vec()
            ],
            file_io_spy.append.arguments.take_all()
        )
    }
}
