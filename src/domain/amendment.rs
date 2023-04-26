
use rbatis::rbdc::datetime::DateTime;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Amendments {
    pub uid: String,
    pub dateDepot: DateTime,
}

impl fmt::Display for Amendments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

crud!(Amendments{});
impl_select_page!(Amendments{select_all_paginated(order_by: &str) => "`ORDER BY [#{order_by}] OFFSET ${page_size} * ${page_no} ROWS FETCH NEXT ${page_size} ROWS ONLY`"});
impl_select_page!(Amendments{select_by_uid_paginated(uid:&str) => "`where uid = #{name}`"});