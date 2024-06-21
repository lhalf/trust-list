use crate::dependencies::{Dependencies, Dependency};
use anyhow::{Context, Error};
use std::io::Write;

pub fn write(output_file: String, dependencies: Dependencies) -> Result<(), Error> {
    let mut file = recreate(output_file)?;
    for dependency in dependencies {
        file.write_all(dependency.to_table_format().as_bytes())
            .expect("unable to write");
        file.write_all(&[b'\n']).expect("unable to write");
    }
    Ok(())
}
fn recreate(output_file: String) -> Result<std::fs::File, Error> {
    //TODO clear if exists

    std::fs::File::create(format!("{}.md", output_file)).context("could not create file")
}

impl Dependency {
    fn to_table_format(&self) -> String {
        format!("|{}|{}|", self.name, self.link)
    }
}
