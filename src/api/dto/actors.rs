use rbatis::rbdc::datetime::DateTime;
use serde_derive::Serialize;
use crate::domain::actor::Actors;
use crate::domain::profession::Professions;
use utoipa::{ToSchema};
use crate::api::dto::actors_addresses::ActorsAddressesDTO;
use crate::api::dto::professions::ProfessionsDTO;
use crate::domain::actor_address::ActorsAddresses;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActorsDTO {
    pub uid: String,
    pub title: String,
    pub surname: String,
    pub name: String,
    pub alpha: String,
    pub trigram: String,
    pub birthdate: Option<DateTime>,
    pub birthplace: Option<String>,
    pub death_date: Option<DateTime>,
    pub uri_hatvp: Option<String>,
    pub profession: Option<ProfessionsDTO>,
    pub addresses: Option<ActorsAddressesDTO>,
}

impl ActorsDTO {
    pub fn from_entities(actors: Vec<(&Actors, Option<&Professions>, Option<&Vec<ActorsAddresses>>)>) -> Vec<ActorsDTO> {
        actors.into_iter()
            .map(|(a, p, ad)| Self::from_entity(a, p, ad))
            .collect()
    }

    pub fn from_entity(actor: &Actors, profession: Option<&Professions>, addresses: Option<&Vec<ActorsAddresses>>) -> ActorsDTO {
        let a = actor.clone();
        Self {
            uid: a.uid,
            title: a.title,
            surname: a.surname,
            name: a.name,
            alpha: a.alpha,
            trigram: a.trigram,
            birthdate: a.birthdate,
            birthplace: a.birthplace,
            death_date: a.death_date,
            uri_hatvp: a.uri_hatvp,
            profession: profession.map(ProfessionsDTO::from_domain),
            addresses: addresses.map(ActorsAddressesDTO::from_entities),
        }
    }
}

