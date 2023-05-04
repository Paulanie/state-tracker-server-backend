use std::collections::HashMap;
use actix_web::{web, get};
use actix_web::web::Json;
use rbatis::sql::{Page, PageRequest};
use crate::api::common::{build_result_page, DatabaseError, PaginationRequest};
use crate::AppState;
use crate::domain::actor::Actors;
use crate::api::dto::actors::ActorsDTO;
use crate::domain::profession::Professions;


#[get("/actors")]
async fn list(
    state: web::Data<AppState>,
    page: web::Query<PaginationRequest>,
) -> Result<Json<Page<ActorsDTO>>, DatabaseError> {
    let mut db_pool = &state.pool.clone();
    let page_request = PageRequest::new(page.page, page.size);
    let ordering = page.ordering.clone().unwrap_or_else(|| "trigram".to_string());
    let sort_order = page.sort_order.as_str();
    let actors = Actors::select_all_paginated(&mut db_pool, &page_request, &ordering, sort_order).await;

    let d = actors.map(|data| async move {
        let profession_ids = data.records.iter().map(|a| a.profession_id.to_string()).collect::<Vec<_>>().join(",");
        let professions = Professions::select_by_uids(&mut db_pool, profession_ids).await
            .unwrap()
            .into_iter()
            .map(|p| (p.id, p))
            .collect::<HashMap<_, _>>();
        let records = data.records.iter()
            .map(|a| ActorsDTO::from_domain(a, professions.get(&a.profession_id)))
            .collect::<Vec<_>>();
        build_result_page(data.page_no, data.page_size, data.total, records)
    })?.await;

    Ok(Json(d))
}

#[get("/actors/{id}")]
async fn get(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Option<Json<Actors>>, DatabaseError> {
    let mut db = &state.pool.clone();
    let uid = path.into_inner();
    let actors = Actors::select_by_uid(&mut db, uid).await;

    Ok(actors?.map(Json))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(get);
}