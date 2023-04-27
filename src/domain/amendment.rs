use rbatis::rbdc::datetime::DateTime;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Amendments {
    pub uid: String,
    #[serde(rename = "examenRef")]
    pub examen_ref: String,
    #[serde(rename = "triAmendement")]
    pub tri_amendement: String,
    #[serde(rename = "texteLegislatifRef")]
    pub texte_legislatif_ref: String,
    #[serde(rename = "dateDepot")]
    pub date_depot: Option<DateTime>,
    #[serde(rename = "datePublication")]
    pub date_publication: Option<DateTime>,
    #[serde(rename = "dateSort")]
    pub date_sort: Option<DateTime>,
    pub etat: String,
    #[serde(rename = "sousEtat")]
    pub sous_etat: Option<String>,
    pub representation: String,
    pub article99: bool,
}

impl fmt::Display for Amendments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

crud!(Amendments{});
impl_select_page!(Amendments{select_all_paginated(order_by: &str, sort_order: &str) => "
    if !sql.contains('count'):
        `order by ${order_by} ${sort_order}, uid offset ${page_size} * ${page_no} rows fetch next ${page_size} rows only --`"});
impl_select!(Amendments{select_by_uid(uid:String) -> Option => "`where uid = #{uid}`"});