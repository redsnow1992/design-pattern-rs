use std::{fs, path::PathBuf};

pub trait DataSource {
    fn write(&mut self, data: String);
    fn read(&self) -> String;
}

pub struct FileDataSource {
    pub name: PathBuf,
}

impl DataSource for FileDataSource {
    fn write(&mut self, data: String) {
        fs::write(&self.name, data).unwrap();
    }

    fn read(&self) -> String {
        fs::read_to_string(&self.name).unwrap()
    }
}