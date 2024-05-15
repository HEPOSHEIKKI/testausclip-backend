use serde_derive::Deserialize;
use std::{fs, process::exit};

use spdlog::prelude::*;



#[derive(Deserialize)]
pub struct Data {
    pub testausclip: Testausclip,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Testausclip {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Database {
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub timeout: u16,
}

pub fn read_config(config_file: &str) -> Data{
    info!("Reading configuration file at {}", &config_file);
    let contents = match fs::read_to_string(&config_file){
        Ok(c) => c,
        Err(_) => {
            error!("Could not read config file `{}`", config_file);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            error!("Invalid configuration file: `{}`", config_file);
            exit(1);
        }
    };
    data
}