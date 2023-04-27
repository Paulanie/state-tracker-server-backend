use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder};
use log::error;
use rbatis::rbdc::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Page {
    pub page: u64,
    pub size: u64,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            page: 1,
            size: 20,
        }
    }
}

pub fn return_data<T>(entity: Result<rbatis::sql::Page<T>, Error>) -> impl Responder
    where
        T: serde::Serialize,
{
    match entity {
        Ok(results) => {
            let body = serde_json::to_string(&results.records).unwrap();
            HttpResponse::Ok().content_type(ContentType::json()).body(body)
        }
        Err(msg) => {
            error!("An error occured : {}", msg);
            HttpResponse::InternalServerError().finish()
        }
    }
}