use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RepositoryFile {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
}

impl RepositoryFile {
    pub(crate) fn create(path: &PathBuf) -> RepositoryFile {
        println!("{:?}", path);
        if !path.exists() {
            panic!("Invalid file path");
        }

        if !path.is_file() {
            panic!("Path if not a file");
        }

        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        RepositoryFile {
            name,
            path: path.clone(),
        }
    }
}

impl TrackedFiles {
    pub(crate) fn new() -> TrackedFiles {
        let files = vec![];
        TrackedFiles { files }
    }

    pub(crate) fn add(&mut self, file: RepositoryFile) -> Result<(), &'static str> {
        if self.files.contains(&file) {
            return Err("File is already tracked");
        }

        Ok(self.files.push(file))
    }

    pub(crate) fn remove(&mut self, name: &str) {
        if let Some(index) = self.files.iter().position(|file| file.name == name) {
            self.files.remove(index);
        } else {
            panic!("File not found.")
        }
    }

    pub(crate) fn is_tracked(&self, file: &RepositoryFile) -> bool {
        self.files.contains(file)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TrackedFiles {
    files: Vec<RepositoryFile>,
}
