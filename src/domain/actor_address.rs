use std::fmt;
use serde_derive::Serialize;
use serde_derive::Deserialize;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorsAddresses {
    pub uid: String,
    pub actor_uid: String,
    #[serde(rename = "type")]
    pub address_type: i32,
    #[serde(rename = "typeName")]
    pub address_type_name: String,
    pub weight: Option<i32>,
    pub affiliate_address: Option<String>,
    pub street_number: Option<String>,
    pub street_name: Option<String>,
    pub zip_code: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

impl fmt::Display for ActorsAddresses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

crud!(ActorsAddresses{});
impl_select!(ActorsAddresses{select_by_uid(uid:String) -> Option => "`where uid = #{uid}`"});
impl_select!(ActorsAddresses{select_by_actor_uid(uid:String) => "`where actorUid = #{uid}`"});
impl_select!(ActorsAddresses{select_by_actor_uids(uids:String) => "`where actorUid in (${uids})`"});