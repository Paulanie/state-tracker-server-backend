use actix_web::{web, get};
use actix_web::web::Json;
use rbatis::sql::{PageRequest};
use crate::api::common::error::DatabaseError;
use crate::api::common::pagination::{AmendmentsPageResult, build_result_page, PaginationRequest};
use crate::api::dto::amendments::AmendmentsDTO;
use crate::AppState;
use crate::domain::amendment::Amendments;

#[utoipa::path(
    params(PaginationRequest),
    responses(
        (status = 200, description = "The amendments found.", body = AmendmentsPageResult),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/amendments")]
pub async fn list(
    state: web::Data<AppState>,
    page: web::Query<PaginationRequest>,
) -> Result<Json<AmendmentsPageResult>, DatabaseError> {
    let mut db = &state.pool.clone();
    let amendments = Amendments::select_all_paginated(
        &mut db,
        &PageRequest::new(page.page, page.size),
        page.ordering.clone().unwrap_or(String::from("deliveryDate")).as_str(),
        page.sort_order.clone().as_str(),
    )
        .await;


    Ok(amendments
        .map(|a| build_result_page(a.page_no, a.page_size, a.total, AmendmentsDTO::from_entities(a.records.iter().collect())))
        .map(Json)?)
}

#[utoipa::path(
    params(
        ("id", description = "The id of the amendment")
    ),
    responses(
        (status = 200, description = "The amendment with the requested ID", body = AmendmentsDTO),
        (status = 404, description = "Amendment not found."),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/amendments/{id}")]
pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Option<Json<AmendmentsDTO>>, DatabaseError> {
    let mut db = &state.pool.clone();
    let uid = path.into_inner();
    let amendment = Amendments::select_by_uid(&mut db, uid).await;

    Ok(amendment?.map(|a| Json(AmendmentsDTO::from_entity(&a))))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(get);
}