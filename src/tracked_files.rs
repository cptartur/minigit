use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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
}

impl FilesTracker for TrackedFiles {
    fn create(files: Vec<RepositoryFile>) -> Self where Self: Sized {
        let files = files.clone();
        TrackedFiles { files }
    }

    fn add(&mut self, file: RepositoryFile) -> Result<(), &'static str> {
        if self.files.contains(&file) {
            return Err("File is already tracked");
        }

        Ok(self.files.push(file))
    }
    fn remove(&mut self, name: &str) {
        if let Some(index) = self.files.iter().position(|file| file.name == name) {
            self.files.remove(index);
        } else {
            panic!("File not found.")
        }
    }

    fn tracked_files(&self) -> &Vec<RepositoryFile> {
        self
    }
}

pub(crate) trait FilesTracker {
    fn create(files: Vec<RepositoryFile>) -> Self where Self: Sized;
    fn add(&mut self, file: RepositoryFile) -> Result<(), &'static str>;
    fn remove(&mut self, name: &str);
    fn tracked_files(&self) -> &Vec<RepositoryFile>;
}

#[derive(Serialize, Deserialize)]
pub struct TrackedFiles {
    files: Vec<RepositoryFile>,
}

impl Deref for TrackedFiles {
    type Target = Vec<RepositoryFile>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}