use std::fmt;
use serde_derive::Serialize;
use serde_derive::Deserialize;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Professions {
    pub id: i32,
    pub name: Option<String>,
    pub family: String,
    pub category: String
}

impl fmt::Display for Professions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

crud!(Professions{});
impl_select!(Professions{select_by_uid(id:i32) -> Option => "`where id = #{id}`"});
impl_select!(Professions{select_by_uids(ids:String) => "`where id in (${ids})`"});