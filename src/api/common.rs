use actix_web::{HttpResponse, Responder};
use log::error;
use rbatis::rbdc::Error;
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Clone, Copy, Default)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    #[default]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

#[derive(Deserialize)]
pub struct Page {
    #[serde(default = "page_default")]
    pub page: u64,
    #[serde(default = "size_default")]
    pub size: u64,
    pub ordering: Option<String>,
    #[serde(default)]
    pub sort_order: SortOrder
}

fn page_default() -> u64 { 1 }
fn size_default() -> u64 { 10 }

pub fn return_data<T>(entity: Result<rbatis::sql::Page<T>, Error>) -> impl Responder
    where
        T: serde::Serialize,
{
    match entity {
        Ok(results) => {
            HttpResponse::Ok().json(&results)
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