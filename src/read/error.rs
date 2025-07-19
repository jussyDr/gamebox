use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct Error(Box<dyn StdError>);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl StdError for Error {}

impl Error {
    pub fn new(err: impl Into<Box<dyn StdError>>) -> Self {
        Self(err.into())
    }
}
