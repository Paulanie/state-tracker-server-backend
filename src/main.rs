extern crate itertools;

mod configuration;
mod migration;
mod domain;
mod api;

use std::process::exit;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web, middleware::Logger, http};
use log::{error, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::configuration::APPCONFIG;
use crate::migration::migrate;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::pg::PgConnection;

use crate::api::{dto, common, amendments, actors};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pool: DbPool,
}

#[derive(OpenApi)]
#[openapi(
paths(
amendments::list_amendments,
amendments::get_amendment,
actors::list_actors,
actors::get_actor
),
components(
schemas(dto::amendments::AmendmentsDTO),
schemas(dto::actors::ActorsDTO),
schemas(dto::professions::ProfessionsDTO),
schemas(dto::actors_addresses::ActorsAddressesDTO),
schemas(dto::actors_addresses::AddressDTO),
schemas(common::pagination::SortOrder),
schemas(common::pagination::ActorsPageResult),
schemas(common::pagination::AmendmentsPageResult)
),
tags(
(name = "todo", description = "Todo management endpoints.")
),
)]
struct ApiDoc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("
   _____ _        _         _______             _
  / ____| |      | |       |__   __|           | |
 | (___ | |_ __ _| |_ ___     | |_ __ __ _  ___| | _____ _ __
  \\___ \\| __/ _` | __/ _ \\    | | '__/ _` |/ __| |/ / _ \\ '__|
  ____) | || (_| | ||  __/    | | | | (_| | (__|   <  __/ |
 |_____/ \\__\\__,_|\\__\\___|    |_|_|  \\__,_|\\___|_|\\_\\___|_|

                                                              ");
    info!("Starting State Tracker Server with the following configuration : \n {}", APPCONFIG.to_string());
    info!("Applying SQL Migrations ...");
    match migrate().await {
        Ok(_) => { info!("Migration successful") }
        Err(error) => {
            error!("Migration failed, error : {}", error.to_string());
            exit(1)
        }
    }

    let cm = ConnectionManager::<PgConnection>::new(APPCONFIG.main.connection_string());
    let pool = Pool::builder().build(cm).unwrap();

    let app_state = AppState {
        pool,
    };

    let openapi = ApiDoc::openapi();

    info!("Starting Actix-Web Server");
    info!("Swagger UI can be found on : http://127.0.0.1:8080/swagger-ui/");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .configure(amendments::config)
            .configure(actors::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}