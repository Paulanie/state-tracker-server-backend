#[macro_use]
extern crate rbatis;
extern crate rbdc_mssql;

mod configuration;
mod domain;
mod migration;
mod api;

use std::process::exit;
use actix_web::{App, HttpServer, web, middleware::Logger};
use rbatis::{Rbatis};
use rbdc_mssql::driver::MssqlDriver;
use log::{error, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::configuration::APPCONFIG;
use crate::migration::migrate;


#[derive(Clone)]
pub struct AppState {
    pool: Rbatis,
}

#[derive(OpenApi)]
    #[openapi(
        paths(
            api::amendments::list,
            api::amendments::get,
            api::actors::list,
            api::actors::get
        ),
        components(
            schemas(api::dto::amendments::AmendmentsDTO),
            schemas(api::dto::actors::ActorsDTO),
            schemas(api::dto::actors::ProfessionsDTO),
            schemas(api::common::SortOrder),
            schemas(api::common::ActorsPageResult),
            schemas(api::common::AmendmentsPageResult),
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
struct ApiDoc;


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

    let openapi = ApiDoc::openapi();

    info!("Starting Actix-Web Server");
    info!("Swagger UI can be found on : http://127.0.0.1:8080/swagger-ui/");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(api::amendments::config)
            .configure(api::actors::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}