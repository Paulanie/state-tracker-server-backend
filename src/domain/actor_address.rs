use std::fmt;

use diesel::prelude::*;



#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::domain::schema::actors_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActorsAddresses {
    pub uid: String,
    pub actor_uid: String,
    pub address_type: i32,
    pub address_type_name: String,
    pub weight: Option<i32>,
    pub affiliate_address: Option<String>,
    pub street_number: String,
    pub street_name: String,
    pub zip_code: String,
    pub city: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

impl fmt::Display for ActorsAddresses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}