use std::fmt;

use diesel::prelude::*;
use diesel::sql_types::Date;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::domain::schema::actors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Actors {
    pub uid: String,
    pub title: String,
    pub surname: String,
    pub name: String,
    pub alpha: String,
    pub trigram: Option<String>,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub death_date: Option<Date>,
    pub uri_hatvp: Option<String>,
    pub profession_id: Option<i32>,
}

impl fmt::Display for Actors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

