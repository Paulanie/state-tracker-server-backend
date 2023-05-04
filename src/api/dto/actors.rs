use rbatis::rbdc::datetime::DateTime;
use serde_derive::Serialize;
use crate::domain::actor::Actors;
use crate::domain::profession::Professions;

#[derive(Serialize)]
pub struct ActorsDTO {
    pub uid: String,
    pub title: String,
    pub surname: String,
    pub name: String,
    pub alpha: String,
    pub trigram: String,
    pub birthdate: Option<DateTime>,
    pub birthplace: Option<String>,
    #[serde(rename = "deathDate")]
    pub death_date: Option<DateTime>,
    #[serde(rename = "uriHatvp")]
    pub uri_hatvp: Option<String>,
    pub profession: Option<ProfessionsDTO>,
}

#[derive(Serialize)]
pub struct ProfessionsDTO {
    pub name: String,
    pub family: String,
    pub category: String,
}

impl ActorsDTO {
    pub fn from_domain(actor: &Actors, profession: Option<&Professions>) -> ActorsDTO {
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
        }
    }
}

impl ProfessionsDTO {
    pub fn from_domain(profession: &Professions) -> ProfessionsDTO {
        let p = profession.clone();
        Self {
            name: p.name,
            family: p.family,
            category: p.category,
        }
    }
}