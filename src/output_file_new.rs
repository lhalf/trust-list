use std::collections::BTreeSet;

pub fn crates_from_output_file(_file_contents: Vec<u8>) -> anyhow::Result<BTreeSet<String>> {
    Err(anyhow::anyhow!("invalid headings in trust list"))
}

#[cfg(test)]
mod tests {
    use crate::output_file_new::crates_from_output_file;

    #[test]
    fn empty_output_file_is_invalid() {
        assert_eq!(
            "invalid headings in trust list",
            crates_from_output_file(b"".to_vec())
                .unwrap_err()
                .root_cause()
                .to_string()
        )
    }

    #[test]
    fn output_file_with_invalid_headings_is_invalid() {
        assert_eq!(
            "invalid headings in trust list",
            crates_from_output_file(b"not|correct|headings".to_vec())
                .unwrap_err()
                .root_cause()
                .to_string()
        )
    }
}
