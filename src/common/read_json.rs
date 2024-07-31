use std::fs;
use super::structs::{Config, DharMannJson};

// read config file
pub fn config() -> Config {
    match fs::read_to_string("./Config.json") {
        Err(e) => {
            tracing::info!("Error occured while trying to parse config: {}", e);
            panic!();
        },
        Ok(file) => {
            let config: Config = serde_json::from_str(&file).expect("JSON was not well-formatted");
            return config
        }
    }
}

// read dhar mann file
pub fn dhar_mann() -> DharMannJson {
    match fs::read_to_string("./assets/Dhar-mann.json") {
        Err(e) => {
            tracing::info!("Error occured while trying to parse config: {}", e);
            panic!();
        },
        Ok(file) => {
            let dhar_mann_json: DharMannJson = serde_json::from_str(&file).expect("JSON was not well-formatted");
            return dhar_mann_json
        }
    }
}