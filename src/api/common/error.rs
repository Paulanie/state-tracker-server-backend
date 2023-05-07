use rbatis::rbdc::Error;
use std::fmt::{Debug, Display, Formatter};


pub struct DatabaseError {
    error: Error,
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl actix_web::error::ResponseError for DatabaseError {}

impl From<Error> for DatabaseError {
    fn from(error: Error) -> Self {
        DatabaseError { error }
    }
}