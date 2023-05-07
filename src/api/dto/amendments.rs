use serde_derive::Serialize;
use rbatis::rbdc::datetime::DateTime;
use crate::domain::amendment::Amendments;
use utoipa::{ToSchema};

#[derive(Serialize, ToSchema)]
pub struct AmendmentsDTO {
    pub uid: String,
    #[serde(rename = "examinationRef")]
    pub examination_ref: String,
    #[serde(rename = "triAmendment")]
    pub tri_amendment: String,
    #[serde(rename = "legislativeTextRef")]
    pub legislative_text_ref: String,
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<DateTime>,
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<DateTime>,
    #[serde(rename = "sortDate")]
    pub sort_date: Option<DateTime>,
    pub state: String,
    #[serde(rename = "subState")]
    pub sub_state: Option<String>,
    pub representation: String,
    pub article99: bool,
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
        }
    }
}