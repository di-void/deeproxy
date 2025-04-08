use crate::utils::hash_url;
use std::{fs, io, path::PathBuf};

// https://serde.rs/
// https://docs.rs/serde/latest/serde/

pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub fn new() -> Self {
        let cache_dir = PathBuf::from("cache");
        fs::create_dir_all(&cache_dir).expect("Couldn't create cache directory");

        Cache { path: cache_dir }
    }

    pub fn get(&self, key: &str) -> io::Result<()> {
        let hashed_key = hash_url(key);
        let _file_path = self.path.join(hashed_key);
        // fs read
        Ok(())
    }

    pub fn set(&self, key: &str) -> io::Result<()> {
        let hashed_key = hash_url(key);
        let _file_path = self.path.join(hashed_key);
        // fs write
        Ok(())
    }

    pub fn clear(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.path)?;
        fs::create_dir_all(&self.path)?;
        Ok(())
    }
}
