mod file;

pub use file::*;

pub mod types {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CachedResponse {
        pub status: u16,
        pub headers: HashMap<String, String>,
        pub body: Vec<u8>,
    }
}
