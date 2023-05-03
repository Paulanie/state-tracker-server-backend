use actix_web::{Responder, web, get};
use rbatis::sql::PageRequest;
use crate::api::common::{Page, return_data, return_single_data};
use crate::AppState;
use crate::domain::actor::Actors;

#[get("/actors")]
async fn list(
    state: web::Data<AppState>,
    page: web::Query<Page>,
) -> impl Responder {
    let mut db = &state.pool.clone();
    let actors = Actors::select_all_paginated(
        &mut db,
        &PageRequest::new(page.page, page.size),
        page.ordering.clone().unwrap_or(String::from("trigram")).as_str(),
        page.sort_order.clone().as_str(),
    )
        .await;

    return_data(actors)
}

#[get("/actors/{id}")]
async fn get(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let mut db = &state.pool.clone();
    let uid = path.into_inner();
    let actors = Actors::select_by_uid(&mut db, uid).await;

    return_single_data(actors)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(get);
}