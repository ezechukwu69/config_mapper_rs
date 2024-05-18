use std::{fs, process::exit};

use super::dto::Data;

pub struct Parser {
    pub path: String,
    pub data: Data
}

impl Parser {
    pub fn new(config_path: Option<String>) -> Self {
        let set_path = config_path.unwrap_or("config_mapper.toml".to_owned());
        let contents_of_file = fs::read_to_string(set_path.clone());

        let contents = match contents_of_file {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        };

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        };

        Self {
            path: set_path,
            data
        }
    }
}
