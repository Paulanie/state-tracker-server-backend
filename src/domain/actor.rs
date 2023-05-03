use std::fmt;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use rbatis::rbdc::datetime::DateTime;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actors {
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
    #[serde(rename = "professionId")]
    pub profession_id: i32
}

impl fmt::Display for Actors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

crud!(Actors{});
impl_select_page!(Actors{select_all_paginated(order_by: &str, sort_order: &str) => "
    if !sql.contains('count'):
        `order by ${order_by} ${sort_order}, uid offset ${page_size} * ${page_no} rows fetch next ${page_size} rows only --`"});
impl_select!(Actors{select_by_uid(uid:String) -> Option => "`where uid = #{uid}`"});