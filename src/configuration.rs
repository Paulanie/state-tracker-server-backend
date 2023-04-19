use std::fs;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub cosmos: CosmosConfig,
}

#[derive(Deserialize)]
pub struct CosmosConfig {
    pub account: String,
    pub key: String,
    pub database: String,
    pub collection: String,
}

lazy_static!(
        pub static ref APPCONFIG: ApplicationConfig = Some(String::from("application.toml"))
            .map(|filepath| fs::read_to_string(filepath).expect("Unable to find Application Configuration file with the given path."))
            .map(|env| toml::from_str::<ApplicationConfig>(env.as_str()).expect("Unable to load Application Configuration data."))
            .expect("An error occurred when loading application configuration file.");
);