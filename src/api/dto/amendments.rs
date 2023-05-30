use serde_derive::Serialize;
use rbatis::rbdc::datetime::DateTime;
use crate::domain::amendment::Amendments;
use utoipa::{ToSchema};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AmendmentsDTO {
    pub uid: String,
    pub examination_ref: String,
    pub tri_amendment: String,
    pub legislative_text_ref: String,
    pub delivery_date: Option<DateTime>,
    pub publication_date: Option<DateTime>,
    pub sort_date: Option<DateTime>,
    pub state: String,
    pub sub_state: Option<String>,
    pub representation: String,
    pub article99: bool,
    pub content_summary: String,
    pub content_title: String,
}

impl AmendmentsDTO {
    pub fn from_entities(actors: Vec<&Amendments>) -> Vec<AmendmentsDTO> {
        actors.into_iter()
            .map(|a| Self::from_entity(a))
            .collect()
    }

    pub fn from_entity(amendment: &Amendments) -> AmendmentsDTO {
        let a = amendment.clone();
        Self {
            uid: a.uid,
            examination_ref: a.examination_ref,
            tri_amendment: a.tri_amendment,
            legislative_text_ref: a.legislative_text_ref,
            delivery_date: a.delivery_date,
            publication_date: a.publication_date,
            sort_date: a.sort_date,
            state: a.state,
            sub_state: a.sub_state,
            representation: a.representation,
            article99: a.article99,
            content_title: a.content_title,
            content_summary: a.content_summary,
        }
    }
}