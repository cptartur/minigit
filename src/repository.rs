use crate::json::JsonSerializer;
use crate::tracked_files::{RepositoryFile, TrackedFiles};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
struct CommittedFile {
    file: RepositoryFile,
    contents: String,
    path: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Commit {
    message: String,
    version: u32,
    files: Vec<CommittedFile>,
}

impl Commit {
    fn create(
        message: &str,
        version: u32,
        files: &Vec<RepositoryFile>,
        commit_path: &PathBuf,
    ) -> Commit {
        let commit_path = commit_path.clone();
        let files = files
            .iter()
            .map(|file| CommittedFile {
                file: file.clone(),
                contents: Repository::read_file_contents(file),
                path: commit_path.clone(),
            })
            .collect();

        Commit {
            message: message.to_string(),
            version,
            files,
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
            .deserialize("tracked_files", None)
            .expect("Failed to load tracked files");

        let mut commits = vec![];
        let paths = fs::read_dir(&base_dir).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap();
                if name.starts_with("COMMIT") {
                    let commit: Commit = serializer.deserialize("meta", Some(&path)).unwrap();
                    commits.push(commit);
                }
            }
        }

        let version: u32 = serializer.deserialize("VERSION", None).unwrap();

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
        self.commit(commit_message);
    }

    fn read_file_contents(file: &RepositoryFile) -> String {
        fs::read_to_string(&file.path).unwrap()
    }

    pub fn remove(&mut self, name: &str) {
        self.tracked_files.remove(name);
    }

    pub fn commit(&mut self, message: Option<&str>) {
        let mut commit_path = self
            .base_dir
            .clone();
        commit_path.push(Path::new(&self.version.to_string()));

        self.version += 1;
        let files = &self.tracked_files;
        let commit = Commit::create(message.unwrap_or("Committed"), self.version, files, &commit_path);
        self.commits.push(commit);
    }

    pub fn history(&self, n_versions: Option<u32>) {
        match n_versions {
            None => self.print_version(self.version),
            Some(versions) => {
                let start = self.version.checked_sub(versions).expect("Too many versions requested") + 1;
                for version in start..=self.version {
                    self.print_version(version);
                }
            }
        }
    }

    fn print_version(&self, version: u32) {
        if version > self.version {
            println!("Incorrect version provided");
        }
        let commit = self
            .commits
            .iter()
            .find(|commit| commit.version == version)
            .expect("Commit not found for version");

        let files_tracked: Vec<String> = commit.files.iter().map(|f| f.file.name.clone()).collect();

        println!("Version {version}. \nCommit message: {}. \nFiles tracked in version: {:?}", commit.message, files_tracked)
    }

    pub fn checkout(self, version: u32) {
        let commit = self
            .commits
            .iter()
            .find(|commit| commit.version == version)
            .expect("Commit not found for version");

        Self::checkout_commit(commit);
    }

    fn checkout_commit(commit: &Commit) {
        for committed_file in &commit.files {
            let mut file = File::create(&committed_file.file.path).unwrap();
            let contents = &committed_file.contents;
            write!(file, "{}", contents).expect("Failed to write to a file");
        }
    }

    pub fn save(&self) {
        let serializer = JsonSerializer::create(&self.base_dir);

        serializer.serialize("tracked_files", &self.tracked_files, None);

        serializer.serialize("VERSION", &self.version, None);

        for commit in &self.commits {
            let dir_name = format!("COMMIT_{}", &commit.version);

            let mut dir = self.base_dir.clone();
            dir.push(dir_name);

            if dir.exists() {
                continue;
            }

            fs::create_dir(&dir).expect("Failed to create commit directory");

            for committed_file in &commit.files {
                let file_name = &committed_file.file.name;
                serializer.serialize(file_name, committed_file, Some(&dir));
            }

            let name = "meta";
            serializer.serialize(name, commit, Some(&dir));
        }
    }

    fn construct_path(base_path: &PathBuf, name: &str) -> PathBuf {
        let mut path = base_path.clone().parent().unwrap().to_path_buf();
        path.push(name);

        path
    }
}
