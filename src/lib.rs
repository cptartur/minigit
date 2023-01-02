use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use std::rc::Rc;

use serde::{Serialize};

struct SerializedItem {
    name: String,
    contents: String,
}

trait Serializable {
    fn serialize(&self) -> SerializedItem;
}

struct Serializer {
    base_dir: PathBuf,
}

impl Serializer {
    fn create(base_dir: PathBuf) -> Serializer {
        Serializer { base_dir }
    }

    fn serialize(&self, serializable: &Vec<Box<dyn Serializable>>) {
        for item in serializable {
            let SerializedItem { name, contents } = item.serialize();

            let mut path = self.base_dir.clone();
            path.push(name);

            let mut file = File::create(path).expect("Failed to create a file.");
            write!(file, "{}", contents).expect("Failed to write to a file.");
        }
    }
}


#[derive(Serialize)]
pub struct RepositoryFile {
    name: String,
    path: PathBuf,
}

impl RepositoryFile {
    fn create(path: PathBuf) -> RepositoryFile {
        if !path.exists() {
            panic!("Invalid file path");
        }

        if !path.is_file() {
            panic!("Path if not a file");
        }

        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        RepositoryFile { name, path }
    }
}

struct TrackedFiles {
    files: Vec<RepositoryFile>,
}

impl TrackedFiles {
    fn new() -> TrackedFiles {
        let files = vec![];
        TrackedFiles { files }
    }

    fn add(&mut self, file: RepositoryFile) {
        self.files.push(file);
    }

    fn remove(&mut self, name: &str) {
        if let Some(index) = self.files.iter().position(|file| file.name == name) {
            self.files.remove(index);
        } else {
            panic!("File not found.")
        }
    }
}

impl Serializable for TrackedFiles {
    fn serialize(&self) -> SerializedItem {
        let contents = serde_json::to_string(&self.files).unwrap();
        let name = "tracked_files_serialized.txt".to_string();
        SerializedItem { name, contents }
    }
}

struct Commit {
    message: String,
    version: u32,
}

impl Commit {
    fn create(message: &str, version: u32) -> Commit {
        Commit { message: message.to_string(), version }
    }
}

pub struct Repository {
    base_dir: PathBuf,
    tracked_files: TrackedFiles,
    commits: Vec<Commit>,
    version: u32,
}

impl Repository {
    pub fn new() -> Repository {
        let base_dir = env::current_dir().expect("Failed to get the current working directory");
        let tracked_files = TrackedFiles::new();
        let commits = vec![];
        let version = 0;

        Repository { base_dir, tracked_files, commits, version }
    }

    pub fn add(&mut self, name: &str, commit_message: Option<&str>) {
        let path = Self::construct_path(&self.base_dir, name);
        let file = RepositoryFile::create(path);
        let name = file.name.as_str();

        let message = commit_message.unwrap_or(format!("Adding a file {name}").as_str()).to_string();
        let commit = Commit { message, version: self.version };
        self.commits.push(commit);

        // self.tracked_files.add(file);
        let val = self.tracked_files.borrow_mut();
        val.add(file);
    }

    pub fn remove(&mut self, name: &str) {
        self.tracked_files.remove(name);
    }

    pub fn commit(name: &str) {}

    pub fn checkout(version: u32) {}

    pub fn serialize(self) {
        let mut serializable: Vec<Box<dyn Serializable>> = vec![];
        serializable.push(Box::new(self.tracked_files));

        let serializer = Serializer::create(self.base_dir.clone());
        serializer.serialize(&serializable);
    }

    fn construct_path(base_path: &PathBuf, name: &str) -> PathBuf {
        let mut path = base_path.clone();
        path.push(name);

        path
    }
}