#[macro_use]
extern crate rbatis;
extern crate rbdc_mssql;

mod data;
mod configuration;
mod domain;

use std::sync::Arc;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use azure_core::Error;
use futures::stream::StreamExt;
use rbatis::Rbatis;
use rbdc_mssql::driver::MssqlDriver;
use crate::configuration::APPCONFIG;
use crate::domain::amendment::Amendment;

#[macro_use]
extern crate lazy_static;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(APPCONFIG.cosmos.database.to_owned())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/amendments")]
async fn get_amendments(
    state: web::Data<AppState>
) -> impl Responder {
    let mut db = &state.pool.clone();
    let amendments = Amendment::select_by_column(&mut db, "uid", "")
        .await
        .map(|data| {
            println!("Data of size : {}", data.len());
            for x in data {
                println!("{}", x);
            }
        });

    match amendments {
        Ok(_) => { HttpResponse::Ok() }
        Err(msg) => { HttpResponse::InternalServerError() }
    }
}

#[derive(Clone)]
struct AppState {
    pool: Rbatis,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rb = Rbatis::new();
    rb.init(MssqlDriver {}, "jdbc:sqlserver://localhost;user=sa;password=Statetracker123;databaseName=StateTracker").expect("Unable to initialize rbatis.");
    let app_state = AppState {
        pool: rb
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(hello)
            .service(echo)
            .service(get_amendments)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}