use rbatis::rbdc::Error;
use serde::{Deserialize};
use std::default::Default;
use std::fmt::{Debug, Display, Formatter};
use serde_derive::Serialize;
use utoipa::{IntoParams, ToSchema};
use crate::api::dto::{actors::{ActorsDTO}, amendments::{AmendmentsDTO}};

#[derive(Deserialize, Clone, Copy, Default, ToSchema)]
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

#[derive(Deserialize, IntoParams)]
pub struct PaginationRequest {
    #[serde(default = "page_default")]
    pub page: u64,
    #[serde(default = "size_default")]
    pub size: u64,
    pub ordering: Option<String>,
    #[serde(default)]
    pub sort_order: SortOrder,
}

fn page_default() -> u64 { 1 }

fn size_default() -> u64 { 10 }

#[derive(Serialize, ToSchema)]
#[aliases(ActorsPageResult = PageResult<ActorsDTO>,
          AmendmentsPageResult = PageResult<AmendmentsDTO>,)]
pub struct PageResult<T> where T: serde::Serialize {
    total: u64,
    pages: u64,
    page_no: u64,
    page_size: u64,
    records: Vec<T>,
}

pub fn build_result_page<T>(page_no: u64, page_size: u64, total: u64, records: Vec<T>) -> PageResult<T> where T: serde::Serialize {
    PageResult {
        total,
        pages: (total as f64 / page_size as f64).ceil() as u64,
        page_no,
        page_size,
        records,
    }
}

pub struct DatabaseError {
    error: Error,
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl actix_web::error::ResponseError for DatabaseError {}

impl From<Error> for DatabaseError {
    fn from(error: Error) -> Self {
        DatabaseError { error }
    }
}