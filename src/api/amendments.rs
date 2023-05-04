use actix_web::{Responder, web, get};
use rbatis::sql::PageRequest;
use crate::api::common::{Page, return_paginated_data, return_single_data};
use crate::AppState;
use crate::domain::amendment::Amendments;

#[get("/amendments")]
async fn list(
    state: web::Data<AppState>,
    page: web::Query<Page>,
) -> impl Responder {
    let mut db = &state.pool.clone();
    let amendments = Amendments::select_all_paginated(
        &mut db,
        &PageRequest::new(page.page, page.size),
        page.ordering.clone().unwrap_or(String::from("deliveryDate")).as_str(),
        page.sort_order.clone().as_str(),
    )
        .await;

    return_paginated_data(amendments)
}

#[get("/amendments/{id}")]
async fn get(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let mut db = &state.pool.clone();
    let uid = path.into_inner();
    let amendment = Amendments::select_by_uid(&mut db, uid).await;

    return_single_data(amendment)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(get);
}