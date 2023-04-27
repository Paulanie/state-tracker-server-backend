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
            HttpResponse::Ok().json(&results.records)
        }
        Err(err) => {
            error!("An error occured : {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn return_single_data<T>(entity: Result<Option<T>, Error>) -> impl Responder
    where
        T: serde::Serialize,
{
    match entity {
        Ok(result) => {
            match result {
                None => HttpResponse::NotFound().finish(),
                Some(e) => HttpResponse::Ok().json(e)
            }
        }
        Err(err) => {
            error!("An error occured : {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}