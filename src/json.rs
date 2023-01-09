use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct JsonSerializer {
    base_dir: PathBuf,
}

type JsonSerializerResult<T> = Result<T, Box<dyn Error>>;

impl JsonSerializer {
    pub(crate) fn create(base_dir: &PathBuf) -> JsonSerializer {
        let base_dir = base_dir.clone();
        JsonSerializer { base_dir }
    }

    pub(crate) fn serialize<S>(&self, name: &str, serializable: &S, arg_path: Option<&Path>)
        where
            S: Serialize,
    {
        let mut path = self.base_dir.clone();

        if let Some(p) = arg_path {
            path.push(p)
        }

        path.push(name);

        let contents = serde_json::to_string(&serializable).unwrap();
        let mut file = File::create(path).unwrap();
        write!(file, "{}", contents).unwrap();
    }

    pub(crate) fn deserialize<T>(&self, name: &str, arg_path: Option<&Path>) -> JsonSerializerResult<T>
        where
            T: DeserializeOwned,
    {
        let mut path = self.base_dir.clone();

        if let Some(p) = arg_path {
            path.push(p)
        }
        
        path.push(name);

        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents).unwrap())
    }
}
