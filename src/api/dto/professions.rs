use serde_derive::Serialize;
use crate::domain::profession::Professions;
use utoipa::{ToSchema};

#[derive(Serialize, ToSchema)]
pub struct ProfessionsDTO {
    pub name: Option<String>,
    pub family: String,
    pub category: String,
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