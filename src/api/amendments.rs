use actix_web::{web, get};
use actix_web::web::Json;
use rbatis::sql::{Page, PageRequest};
use crate::api::common::{DatabaseError, PaginationRequest};
use crate::AppState;
use crate::domain::amendment::Amendments;

#[get("/amendments")]
async fn list(
    state: web::Data<AppState>,
    page: web::Query<PaginationRequest>,
) -> Result<Json<Page<Amendments>>, DatabaseError> {
    let mut db = &state.pool.clone();
    let amendments = Amendments::select_all_paginated(
        &mut db,
        &PageRequest::new(page.page, page.size),
        page.ordering.clone().unwrap_or(String::from("deliveryDate")).as_str(),
        page.sort_order.clone().as_str(),
    )
        .await;

    Ok(Json(amendments?))
}

#[get("/amendments/{id}")]
async fn get(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<Option<Amendments>>, DatabaseError> {
    let mut db = &state.pool.clone();
    let uid = path.into_inner();
    let amendment = Amendments::select_by_uid(&mut db, uid).await;

    Ok(Json(amendment?))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(get);
}