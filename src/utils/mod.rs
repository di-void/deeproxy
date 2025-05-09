mod logger;
// pub use logger::*;

use sha2::{Digest, Sha256};

pub fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    format!("{:x}", hasher.finalize())
}
