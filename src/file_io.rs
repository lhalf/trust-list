use anyhow::Context;
use std::io::Write;
use std::path::PathBuf;

pub struct OutputFile {
    pub path: PathBuf,
}

impl OutputFile {
    pub fn at(path: PathBuf) -> Self {
        Self { path }
    }
}

pub trait FileIO {
    fn exists(&self) -> bool;
    fn create(&self) -> anyhow::Result<()>;
    fn append(&self, contents: &[u8]) -> anyhow::Result<()>;
    fn read_to_string(&self) -> anyhow::Result<String>;
}

impl FileIO for OutputFile {
    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn create(&self) -> anyhow::Result<()> {
        std::fs::File::create(&self.path).context("failed to create output file")?;
        Ok(())
    }

    fn append(&self, contents: &[u8]) -> anyhow::Result<()> {
        std::fs::OpenOptions::new()
            .append(true)
            .open(&self.path)
            .context("output file does not exist")?
            .write_all(contents)
            .context("unable to write to output file")
    }

    fn read_to_string(&self) -> anyhow::Result<String> {
        std::fs::read_to_string(&self.path).context("failed to read output file")
    }
}
