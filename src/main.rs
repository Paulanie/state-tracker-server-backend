mod data;
mod configuration;

use std::sync::Arc;
use coi_actix_web::{AppExt, inject};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use coi::container;
use configuration::ApplicationConfigProvider;
use configuration::ApplicationConfig;

#[get("/")]
#[coi_actix_web::inject]
async fn hello(
    #[inject] appconfig: Arc<ApplicationConfig>
) -> impl Responder {
    HttpResponse::Ok().body(appconfig.cosmos.database.to_owned())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let container = container!{
        appconfig => ApplicationConfigProvider; scoped
    };

    HttpServer::new(move || {
        App::new()
            .register_container(container.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}