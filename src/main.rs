#[macro_use]
extern crate rbatis;
extern crate rbdc_mssql;

mod configuration;
mod domain;
mod migration;
mod api;

use std::process::exit;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web, middleware::Logger};
use rbatis::{Rbatis};
use rbdc_mssql::driver::MssqlDriver;
use log::{error, info};
use crate::configuration::APPCONFIG;
use crate::migration::migrate;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello !")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[derive(Clone)]
struct AppState {
    pool: Rbatis,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("
   _____ _        _         _______             _
  / ____| |      | |       |__   __|           | |
 | (___ | |_ __ _| |_ ___     | |_ __ __ _  ___| | _____ _ __
  \\___ \\| __/ _` | __/ _ \\    | | '__/ _` |/ __| |/ / _ \\ '__|
  ____) | || (_| | ||  __/    | | | | (_| | (__|   <  __/ |
 |_____/ \\__\\__,_|\\__\\___|    |_|_|  \\__,_|\\___|_|\\_\\___|_|

                                                              ");
    info!("Starting State Tracker Server");
    info!("Applying SQL Migrations ...");
    match migrate().await {
        Ok(_) => { info!("Migration successful") }
        Err(error) => {
            error!("Migration failed, error : {}", error.to_string());
            exit(1)
        }
    }

    let rb = Rbatis::new();
    rb.init(MssqlDriver {}, APPCONFIG.main.db_connection_string().to_owned().as_str()).unwrap();
    match rb.exec("SELECT CURRENT_USER", vec!()).await {
        Ok(_) => { info!("Connection to database successful") }
        Err(error) => {
            error!("Unable to query database, error : {}", error.to_string());
            exit(1)
        }
    }

    let app_state = AppState {
        pool: rb,
    };

    info!("Starting Actix-Web Server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(hello)
            .configure(api::amendments::config)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}