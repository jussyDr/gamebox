use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use super::{Readable, Result};

pub struct ReadConfig {
    header_options: HeaderOptions,
    body_options: BodyOptions,
}

impl ReadConfig {
    pub fn new() -> Self {
        Self {
            header_options: HeaderOptions::default(),
            body_options: BodyOptions::default(),
        }
    }

    pub fn read_header(mut self, header_options: HeaderOptions) -> Self {
        self.header_options = header_options;
        self
    }

    pub fn read_body(mut self, body_options: BodyOptions) -> Self {
        self.body_options = body_options;
        self
    }

    pub fn read<T: Readable>(&self, reader: impl Read) -> Result<T> {
        T::read(reader, self.header_options, self.body_options)
    }

    pub fn read_file<T: Readable>(&self, path: impl AsRef<Path>) -> Result<T> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.read(reader)
    }
}

/// Options for reading the header.
#[derive(Clone, Copy)]
pub enum HeaderOptions {
    /// Should read the body.
    Read {
        /// Set whether or not read heavy chunks.
        ///
        /// Set to `true` by default.
        read_heavy_chunks: bool,
    },
    /// Should skip reading the header.
    Skip {
        /// Assume that the header size field is zero.
        ///
        /// This option exists for reading nodes extracted with
        /// the hook extract option using OpenPlanet, which sets
        /// the header size field to an incorrect value.
        assume_size_zero: bool,
    },
}

impl Default for HeaderOptions {
    fn default() -> Self {
        Self::Read {
            read_heavy_chunks: true,
        }
    }
}

/// Options for reading the body.
#[derive(Clone, Copy)]
pub enum BodyOptions {
    /// Should read the body.
    Read {
        /// Set whether or not to read skippable chunks.
        ///
        /// Set to `true` by default.
        read_skippable_chunks: bool,
    },
    /// Should skip reading the body.
    Skip,
}

impl Default for BodyOptions {
    fn default() -> Self {
        Self::Read {
            read_skippable_chunks: true,
        }
    }
}
