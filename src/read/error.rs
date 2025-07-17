use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct Error(Box<dyn StdError>);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl StdError for Error {}

impl Error {
    pub fn new(err: impl Into<Box<dyn StdError>>) -> Self {
        Self(err.into())
    }

    pub fn expected(name: &str) -> Self {
        todo!()
    }

    pub fn expected_non_null(name: &str) -> Self {
        todo!()
    }

    pub fn index_out_of_bounds(name: &str) -> Self {
        todo!()
    }

    pub fn unknown(name: &str) -> Self {
        todo!()
    }

    pub fn unknown_version(name: &str, version: u32) -> Self {
        todo!()
    }

    pub fn unknown_file_format() -> Self {
        todo!()
    }

    pub fn unknown_chunk_version(version: u32) -> Self {
        todo!()
    }

    pub fn zero(name: &str) -> Self {
        todo!()
    }
}
