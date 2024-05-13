use serde_derive::Deserialize;
use std::{fs, process::exit};



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
}

pub fn read_config(config_file: &str) -> Data{
    let contents = match fs::read_to_string(&config_file){
        Ok(c) => c,
        Err(_) => {
            println!("Could not read config file `{}`", config_file);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            println!("Invalid configuration file: `{}`", config_file);
            exit(1);
        }
    };
    data
}