use rbatis::rbdc::datetime::DateTime;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Amendments {
    pub uid: String,
    pub examenRef: String,
    pub triAmendement: String,
    pub texteLegislatifRef: String,
    pub dateDepot: DateTime,
    pub datePublication: DateTime,
    pub dateSort: Option<DateTime>,
    pub etat: String,
    pub sousEtat: String,
    pub representation: String,
    pub article99: bool,
}

impl fmt::Display for Amendments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

crud!(Amendments{});
impl_select_page!(Amendments{select_all_paginated(order_by: &str) => "
    if !sql.contains('count'):
        `order by ${order_by} offset ${page_size} * ${page_no} rows fetch next ${page_size} rows only --`"});
impl_select_page!(Amendments{select_by_uid_paginated(uid:&str) => "`where uid = #{name}`"});