use std::fs;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub cosmos: CosmosConfig,
    pub main: MainConfig,
}

#[derive(Deserialize)]
pub struct MainConfig {
    pub db_type: String,
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
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

impl MainConfig {
    pub fn db_connection_string(&self) -> String {
        format!("jdbc:sqlserver://{}:{};user={};password={};databaseName={}",
                self.db_host,
                self.db_port,
                self.db_user,
                self.db_pass,
                self.db_name)
    }
}