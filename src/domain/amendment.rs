
use rbatis::rbdc::datetime::DateTime;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Amendment {
    pub uid: String,
}

impl fmt::Display for Amendment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

crud!(Amendment{});