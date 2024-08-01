use std::collections::BTreeSet;
use std::io::Write;

use anyhow::{Context, Error};

use crate::crates_io::Crate;

pub struct OutputFile {
    pub path: String,
}

impl OutputFile {
    pub fn at_path(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    pub fn write_headings(&self) -> Result<(), Error> {
        let mut file = self.open_file_to_append()?;

        file.write_all(Crate::table_heading().as_bytes())
            .expect("unable to write");

        file.write_all(Crate::table_gap().as_bytes())
            .expect("unable to write");

        Ok(())
    }

    pub fn write_row(&self, _crate: Crate) -> Result<(), Error> {
        let mut file = self.open_file_to_append()?;

        file.write_all(_crate.table_entry().as_bytes())
            .with_context(|| format!("failed to write info for {} to file", _crate.name))
    }

    fn open_file_to_append(&self) -> Result<std::fs::File, Error> {
        std::fs::OpenOptions::new()
            .append(true)
            .open(&self.path)
            .context("file does not exist")
    }

    pub fn recreate(&self) -> Result<std::fs::File, Error> {
        std::fs::File::create(&self.path).context("could not create file")
    }

    pub fn get_unchecked_crates(
        &self,
        all_crates: &BTreeSet<String>,
    ) -> Result<BTreeSet<String>, Error> {
        Ok(all_crates
            .difference(
                &self
                    .crates_from_md_table()
                    .context("failed to get existing crates")?,
            )
            .cloned()
            .collect())
    }

    fn crates_from_md_table(&self) -> Result<BTreeSet<String>, Error> {
        if !self.exists() {
            return Err(anyhow::anyhow!("file does not exist"));
        }

        let contents = std::fs::read_to_string(&self.path).context("failed to open file")?;

        Ok(contents
            .split('\n')
            .skip(2)
            .flat_map(|line| line.split('|').skip(1).take(1).collect::<Vec<&str>>())
            .map(|crate_name| crate_name.trim().to_string())
            .collect())
    }
}

impl Crate {
    fn table_heading() -> String {
        // how to do?
        "|crate|downloads|releases|created|last updated|link|\n".to_string()
    }

    fn table_gap() -> String {
        // also how to do?
        "|-|-|-|-|-|-|\n".to_string()
    }

    fn table_entry(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|{}|\n",
            self.name,
            self.downloads,
            self.versions.len(),
            self.created_at.format("%d/%m/%Y"),
            self.updated_at.format("%d/%m/%Y"),
            self.repository
        )
    }
}
