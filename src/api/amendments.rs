use actix_web::{HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use rbatis::sql::PageRequest;
use crate::api::common::Page;
use crate::AppState;
use crate::domain::amendment::Amendments;

async fn list(
    state: web::Data<AppState>,
    page: web::Query<Page>
) -> impl Responder {
    let mut db = &state.pool.clone();
    let amendments = Amendments::select_all_paginated_by_date_depot(&mut db, &PageRequest::new(page.page, page.size))
        .await;

    match amendments {
        Ok(results) => {
            let body = serde_json::to_string(&results.records).unwrap();
            HttpResponse::Ok().content_type(ContentType::json()).body(body)
        }
        Err(msg) => { HttpResponse::InternalServerError().body(msg.to_string()) }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/amendments")
            .route(web::get().to(list))
    );
}