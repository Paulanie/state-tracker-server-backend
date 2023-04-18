use std::fs;
use coi::Inject;
use serde_derive::Deserialize;

#[derive(Deserialize, Inject)]
#[coi(provides pub ApplicationConfig with ApplicationConfig::new(None))]
pub struct ApplicationConfig {
    pub cosmos: CosmosConfig,
}

#[derive(Deserialize)]
pub struct CosmosConfig {
    pub key: String,
    pub database: String,
}

impl ApplicationConfig {
    fn new(path: Option<String>) -> Self {
        println!("Loading configuration ...");
        path
            .or(Some(String::from("application.toml")))
            .map(|filepath| fs::read_to_string(filepath).expect("Unable to find Application Configuration file with the given path."))
            .map(|env| toml::from_str::<ApplicationConfig>(env.as_str()).expect("Unable to load Application Configuration data."))
            .expect("An error occurred when loading application configuration file.")
    }
}