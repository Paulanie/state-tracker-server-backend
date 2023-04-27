use actix_web::{Responder, web};
use rbatis::sql::PageRequest;
use crate::api::common::{Page, return_data};
use crate::AppState;
use crate::domain::amendment::Amendments;

async fn list(
    state: web::Data<AppState>,
    page: web::Query<Page>,
) -> impl Responder {
    let mut db = &state.pool.clone();
    let amendments = Amendments::select_all_paginated_by_date_depot(&mut db, &PageRequest::new(page.page, page.size))
        .await;

    return_data(amendments)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/amendments")
            .route(web::get().to(list))
    );
}