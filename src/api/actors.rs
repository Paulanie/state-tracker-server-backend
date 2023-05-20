use std::collections::HashMap;
use actix_web::{web, get};
use actix_web::web::Json;
use itertools::Itertools;
use rbatis::sql::{PageRequest};
use crate::api::common::error::DatabaseError;
use crate::api::common::pagination::{ActorsPageResult, build_result_page, PaginationRequest};
use crate::AppState;
use crate::domain::actor::Actors;
use crate::api::dto::actors::{ActorsDTO};
use crate::domain::actor_address::ActorsAddresses;
use crate::domain::profession::Professions;

#[utoipa::path(
params(PaginationRequest),
responses(
(status = 200, description = "The actors found.", body = ActorsPageResult),
(status = 500, description = "Internal server error.")
)
)]
#[get("/actors")]
async fn list_actors(
    state: web::Data<AppState>,
    page: web::Query<PaginationRequest>,
) -> Result<Json<ActorsPageResult>, DatabaseError> {
    let mut db_pool = &state.pool.clone();
    let page_request = PageRequest::new(page.page, page.size);
    let ordering = page.ordering.clone().unwrap_or_else(|| "trigram".to_string());
    let sort_order = page.sort_order.as_str();
    let actors = Actors::select_all_paginated(&mut db_pool, &page_request, &ordering, sort_order).await;

    let d = actors.map(|data| async move {
        if data.total <= 0 {
            return build_result_page(data.page_no, data.page_size, data.total, vec![]);
        }
        let profession_ids = data.records.iter().map(|a| a.profession_id.to_string()).collect::<Vec<_>>().join(",");
        let professions = Professions::select_by_uids(&mut db_pool, profession_ids).await
            .unwrap()
            .into_iter()
            .map(|p| (p.id, p))
            .collect::<HashMap<_, _>>();
        let addresses = ActorsAddresses::select_by_actor_uids(&mut db_pool, data
            .records
            .iter()
            .map(|d| format!("'{}'", d.uid))
            .collect::<Vec<_>>()
            .join(","),
        )
            .await
            .unwrap()
            .into_iter()
            .group_by(|a| a.actor_uid.clone())
            .into_iter()
            .map(|(k, v)| (k, v.collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>();
        let records = data.records.iter()
            .map(|a| ActorsDTO::from_entity(a, professions.get(&a.profession_id), addresses.get(&a.uid)))
            .collect::<Vec<_>>();
        let r = build_result_page(data.page_no, data.page_size, data.total, records);
        r
    })?.await;

    Ok(Json(d))
}

#[utoipa::path(
params(
("id", description = "The id of the actor")
),
responses(
(status = 200, description = "The actor with the requested ID", body = ActorsDTO),
(status = 404, description = "Actor not found"),
(status = 500, description = "Internal server error")
)
)]
#[get("/actors/{id}")]
async fn get_actor(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Option<Json<ActorsDTO>>, DatabaseError> {
    let mut db = &state.pool.clone();
    let uid = path.into_inner();
    let actor = Actors::select_by_uid(&mut db, uid).await?;

    Ok(match actor {
        None => None,
        Some(a) => {
            let profession = Professions::select_by_uid(&mut db, a.profession_id).await?;
            let addresses = ActorsAddresses::select_by_actor_uid(&mut db, a.uid.clone()).await?;
            Some(Json(ActorsDTO::from_entity(&a, profession.as_ref(), Some(&addresses))))
        }
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_actors)
        .service(get_actor);
}