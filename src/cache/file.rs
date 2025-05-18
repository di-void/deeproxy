use crate::utils::hash_url;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{io, io::ErrorKind, path::PathBuf};
use tokio::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

// https://serde.rs/
// https://docs.rs/serde/latest/serde/
// https://github.com/serde-rs/json

pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub async fn new() -> Self {
        let cache_dir = PathBuf::from("cache");

        fs::create_dir_all(&cache_dir)
            .await
            .expect("Couldn't create cache directory");

        Cache { path: cache_dir }
    }

    pub async fn get(&self, key: &str) -> io::Result<Option<CachedResponse>> {
        let hashed_key = hash_url(key);
        let file_path = self.path.join(hashed_key);
        let res = fs::read(file_path).await;

        match res {
            Ok(contents) => {
                let res = String::from_utf8(contents).unwrap();
                let serialized = serde_json::from_str::<CachedResponse>(&res).unwrap();
                Ok(Some(serialized))
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn set(&self, key: &str, val: CachedResponse) -> io::Result<Option<CachedResponse>> {
        let hashed_key = hash_url(key);
        let file_path = self.path.join(hashed_key);
        let serialized = serde_json::to_string(&val).unwrap();

        let _ = fs::write(file_path, serialized).await;

        Ok(Some(val))
    }

    pub async fn clear(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.path).await?;
        fs::create_dir_all(&self.path).await?;
        Ok(())
    }
}
