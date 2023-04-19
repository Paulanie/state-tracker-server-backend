mod data;
mod configuration;

use std::sync::Arc;
use coi_actix_web::{AppExt, inject};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use azure_core::Error;
use azure_data_cosmos::prelude::{QueryDocumentsResponse, QueryDocuments};
use coi::container;
use crate::data::cosmos::{Cosmos};
use crate::data::entities::Amendment;
use futures::stream::StreamExt;
use crate::configuration::APPCONFIG;

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
    cosmos: web::Data<Cosmos>
) -> impl Responder {
    let mut response_stream = cosmos.get(Some(1), None);
    while let Some(document) = response_stream.next().await {
        match document {
            Ok(d) => {
                println!("{:?}", d);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cosmos = web::Data::new({ Cosmos::new(
            APPCONFIG.cosmos.account.clone(),
            APPCONFIG.cosmos.key.clone(),
            APPCONFIG.cosmos.database.clone(),
            APPCONFIG.cosmos.collection.clone(),
        ) });

    HttpServer::new(move || {
        App::new()
            .app_data(cosmos.clone())
            .service(hello)
            .service(echo)
            .service(get_amendments)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}