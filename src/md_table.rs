use std::io::Write;

use anyhow::{Context, Error};

use crate::crate_info::Crate;

pub fn write(output_file: String, crates: Vec<Crate>) -> Result<(), Error> {
    let mut file = recreate(output_file)?;

    file.write_all(Crate::table_heading().as_bytes())
        .expect("unable to write");

    file.write_all(Crate::table_gap().as_bytes())
        .expect("unable to write");

    for _crate in crates {
        file.write_all(_crate.table_entry().as_bytes())
            .expect("unable to write");
    }

    Ok(())
}

fn recreate(output_file: String) -> Result<std::fs::File, Error> {
    //TODO clear if exists

    std::fs::File::create(format!("{}.md", output_file)).context("could not create file")
}

impl Crate {
    fn table_heading() -> String {
        // how to do?
        "|name|downloads|created_at|last_updated|link|\n".to_string()
    }

    fn table_gap() -> String {
        // also how to do?
        "|-|-|-|-|-|\n".to_string()
    }

    fn table_entry(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|\n",
            self.name, self.downloads, self.created_at, self.updated_at, self.repository
        )
    }
}
