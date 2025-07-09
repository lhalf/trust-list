use crate::crates_io::get_crate_info;
use crate::file_io::FileIO;
use crate::github::get_contributor_count;
use crate::http_client::GetRequest;
use crate::progress_bar::ProgressBar;
use anyhow::Error;
use itertools::Itertools;
use std::collections::BTreeSet;

pub fn generate_list(
    crate_names: BTreeSet<String>,
    output_file: &impl FileIO,
    http_client: &impl GetRequest,
    progress_bar: &mut impl ProgressBar,
) -> Result<(), Error> {
    let existing_names = parse_existing_crate_names(&output_file.read_to_string()?);
    let missing_names = crate_names.difference(&existing_names);
    progress_bar.set_total(missing_names.try_len().unwrap_or(0) as u64);

    for crate_name in missing_names {
        progress_bar.set_message(&format!("{crate_name} "));
        match get_crate_info(http_client, crate_name) {
            Ok(mut crate_info) => {
                crate_info.contributors =
                    get_contributor_count(http_client, &crate_info.repository).unwrap_or(0);

                output_file.append(crate_info.table_entry().as_bytes())?;
            }
            Err(error) => {
                println!("failed to get info for {crate_name}: {error}");
            }
        }
        progress_bar.increment();
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
    use crate::progress_bar::ProgressBarSpy;
    use std::collections::BTreeSet;

    #[test]
    fn output_file_exists_but_cant_be_read() {
        let crates = BTreeSet::new();
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();
        let mut progress_bar_spy = ProgressBarSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert_eq!(
            "deliberate test error",
            generate_list(
                crates,
                &file_io_spy,
                &http_client_spy,
                &mut progress_bar_spy
            )
            .unwrap_err()
            .to_string()
        )
    }

    #[test]
    fn no_crates_are_required() {
        let crates = BTreeSet::new();
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();
        let mut progress_bar_spy = ProgressBarSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        progress_bar_spy.set_total.returns.push_back(());

        assert!(
            generate_list(
                crates,
                &file_io_spy,
                &http_client_spy,
                &mut progress_bar_spy
            )
            .is_ok()
        )
    }

    #[test]
    fn single_crate_required_get_crate_info_fails() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();
        let mut progress_bar_spy = ProgressBarSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        progress_bar_spy.set_total.returns.push_back(());
        progress_bar_spy.set_message.returns.push_back(());

        http_client_spy
            .get
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        progress_bar_spy.increment.returns.push_back(());

        assert!(
            generate_list(
                crates,
                &file_io_spy,
                &http_client_spy,
                &mut progress_bar_spy
            )
            .is_ok()
        )
    }

    #[test]
    fn single_crate_required_get_reverse_dependencies_fails() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();
        let mut progress_bar_spy = ProgressBarSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        progress_bar_spy.set_total.returns.push_back(());
        progress_bar_spy.set_message.returns.push_back(());

        http_client_spy
            .get
            .returns
            .push_back(Ok(include_str!("../tests/data/crate_info.json").to_string()));
        http_client_spy
            .get
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        progress_bar_spy.increment.returns.push_back(());

        assert!(
            generate_list(
                crates,
                &file_io_spy,
                &http_client_spy,
                &mut progress_bar_spy
            )
            .is_ok()
        )
    }

    #[test]
    fn single_crate_required_get_contributor_count_fails_appends_line_with_0_as_contributor_count()
    {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();
        let mut progress_bar_spy = ProgressBarSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        progress_bar_spy.set_total.returns.push_back(());
        progress_bar_spy.set_message.returns.push_back(());

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

        file_io_spy.append.returns.push_back(Ok(()));

        progress_bar_spy.increment.returns.push_back(());

        assert!(
            generate_list(
                crates,
                &file_io_spy.clone(),
                &http_client_spy,
                &mut progress_bar_spy
            )
            .is_ok()
        );
        assert_eq!(
            vec![
                b"|autospy|1861|0|32|8|15/05/2025|01/07/2025|https://github.com/lhalf/autospy|\n"
                    .to_vec()
            ],
            file_io_spy.append.arguments.take_all()
        )
    }

    #[test]
    fn single_crate_required_appending_table_fails() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();
        let mut progress_bar_spy = ProgressBarSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        progress_bar_spy.set_total.returns.push_back(());
        progress_bar_spy.set_message.returns.push_back(());

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
            generate_list(
                crates,
                &file_io_spy,
                &http_client_spy,
                &mut progress_bar_spy
            )
            .unwrap_err()
            .to_string()
        )
    }

    #[test]
    fn single_crate_required_appends_expected_line_to_table() {
        let crates = BTreeSet::from(["autospy".to_string()]);
        let file_io_spy = FileIOSpy::default();
        let http_client_spy = GetRequestSpy::default();
        let mut progress_bar_spy = ProgressBarSpy::default();

        file_io_spy
            .read_to_string
            .returns
            .push_back(Ok(String::new()));

        progress_bar_spy.set_total.returns.push_back(());
        progress_bar_spy.set_message.returns.push_back(());

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

        progress_bar_spy.increment.returns.push_back(());

        assert!(
            generate_list(
                crates,
                &file_io_spy.clone(),
                &http_client_spy,
                &mut progress_bar_spy
            )
            .is_ok()
        );
        assert_eq!(
            vec![
                b"|autospy|1861|5|32|8|15/05/2025|01/07/2025|https://github.com/lhalf/autospy|\n"
                    .to_vec()
            ],
            file_io_spy.append.arguments.take_all()
        )
    }
}
