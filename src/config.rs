use std::path::Path;
use std::fs::{File};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub consumer_key : String, 
    pub consumer_secret : String,
    pub access_key : String,
    pub access_secret : String,
    pub cron_expression: String, 
}

impl Config {
    pub fn read(path_file: &Path) -> Option<Config> {
        let mut file = match File::open(path_file) {
            Ok(f) => f,
            Err(_) => return None,
        };
        serde_json::from_reader(&mut file).ok()
    }
}
