use crate::json::JsonSerializer;
use crate::tracked_files::{RepositoryFile, TrackedFiles};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Serialize, Deserialize)]
struct Commit {
    message: String,
    version: u32,
    contents: String,
    path: PathBuf,
}

impl Commit {
    fn create(message: &str, version: u32, contents: &str, path: &PathBuf) -> Commit {
        Commit {
            message: message.to_string(),
            version,
            contents: contents.to_string(),
            path: path.clone(),
        }
    }
}

pub struct Repository {
    base_dir: PathBuf,
    tracked_files: TrackedFiles,
    commits: Vec<Commit>,
    version: u32,
}

impl Repository {
    pub fn create() -> Result<Repository, Box<dyn Error>> {
        let mut base_dir = env::current_dir().expect("Failed to get the current working directory");
        base_dir.push(".minigit");
        if base_dir.exists() {
            return Err("Directory for minigit already exits".into());
        }
        fs::create_dir(&base_dir)?;

        let tracked_files = TrackedFiles::new();
        let commits = vec![];
        let version = 0;

        Ok(Repository {
            base_dir,
            tracked_files,
            commits,
            version,
        })
    }

    pub fn load() -> Result<Repository, Box<dyn Error>> {
        let mut base_dir = env::current_dir().expect("Failed to get the current working directory");
        base_dir.push(".minigit");
        if !base_dir.exists() {
            return Err("Directory for minigit does not exit".into());
        }

        let serializer = JsonSerializer::create(&base_dir);
        let tracked_files: TrackedFiles = serializer
            .deserialize("tracked_files")
            .expect("Failed to load tracked files");

        let mut commits = vec![];
        let paths = fs::read_dir(&base_dir).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let name = &path.file_name().unwrap().to_str().unwrap();
            if name.starts_with("COMMIT") {
                let commit: Commit = serializer.deserialize(name).unwrap();
                commits.push(commit);
            }
        }

        let version: u32 = serializer.deserialize("VERSION").unwrap();

        Ok(Repository {
            base_dir,
            tracked_files,
            commits,
            version,
        })
    }

    pub fn add(&mut self, name: &str, commit_message: Option<&str>) {
        let path = Self::construct_path(&self.base_dir, name);
        let file = RepositoryFile::create(&path);

        self.tracked_files.add(file.clone()).unwrap();
        self.commit_file(
            &file,
            commit_message.unwrap_or(format!("Adding a file {}", &file.name).as_str()),
        )
            .unwrap();
    }

    fn read_file_contents(file: &RepositoryFile) -> String {
        fs::read_to_string(&file.path).unwrap()
    }

    pub fn remove(&mut self, name: &str) {
        self.tracked_files.remove(name);
    }

    fn commit_file(
        &mut self,
        file: &RepositoryFile,
        commit_message: &str,
    ) -> Result<(), &'static str> {
        if !self.tracked_files.is_tracked(file) {
            return Err("File is not tracked");
        }

        let contents = Self::read_file_contents(file);

        let message = commit_message;
        self.version += 1;
        let commit = Commit::create(message, self.version, &contents, &file.path);

        Ok(self.commits.push(commit))
    }

    pub fn commit(&mut self, name: &str, commit_message: Option<&str>) {
        let path = Self::construct_path(&self.base_dir, name);
        let file = RepositoryFile::create(&path);

        self.commit_file(
            &file,
            commit_message.unwrap_or(format!("Committing a file {}", &file.name).as_str()),
        )
            .unwrap();
    }

    pub fn checkout(self, version: u32) {
        let commit = self
            .commits
            .iter()
            .find(|commit| commit.version == version)
            .expect("Commit not found for version");
        let contents = &commit.contents;
        let path = &commit.path;

        let mut file = File::create(path).unwrap();
        write!(file, "{}", contents).expect("File to write to a file");
    }

    pub fn save(&self) {
        let serializer = JsonSerializer::create(&self.base_dir);

        serializer.serialize("tracked_files", &self.tracked_files);

        serializer.serialize("VERSION", &self.version);

        for commit in &self.commits {
            let file_name = format!("COMMIT_{}", &commit.version);
            serializer.serialize(&file_name, commit);
        }
    }

    fn construct_path(base_path: &PathBuf, name: &str) -> PathBuf {
        let mut path = base_path.clone().parent().unwrap().to_path_buf();
        path.push(name);

        path
    }
}
