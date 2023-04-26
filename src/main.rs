#[macro_use]
extern crate rbatis;
extern crate rbdc_mssql;

mod configuration;
mod domain;
mod migration;
mod api;

use std::sync::Arc;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web, middleware::Logger};
use futures::stream::StreamExt;
use rbatis::Rbatis;
use rbdc_mssql::driver::MssqlDriver;
use crate::configuration::APPCONFIG;
use crate::domain::amendment::Amendments;
use crate::migration::migrate;

#[macro_use]
extern crate lazy_static;

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

    migrate().await.unwrap();

    let rb = Rbatis::new();
    rb.init(MssqlDriver {}, APPCONFIG.main.db_connection_string().to_owned().as_str()).expect("Unable to initialize rbatis.");

    let app_state = AppState {
        pool: rb,
    };

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