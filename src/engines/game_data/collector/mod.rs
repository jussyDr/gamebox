//! Types used for reading and writing [Collector] nodes.

use std::rc::Rc;

mod read;
mod write;

/// Base class of nodes that can be displayed in an icon tree.
///
/// For example, the blocks, items, and macroblock classes all extend from this class.
#[derive(Default)]
pub struct Collector {
    name: String,
    icon: Icon,
    collection: Option<Rc<str>>,
    description: String,
}

impl Collector {
    /// Name of this collector.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Collection this collector is part of.
    pub fn collection(&self) -> Option<&str> {
        self.collection.as_ref().map(|x| x as _)
    }

    /// Description of this collector.
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Icon of a collector.
pub enum Icon {
    /// Icon is stored as raw ARGB data.
    Argb {
        /// Width of the icon.
        width: u16,
        /// Height of the icon.
        height: u16,
        /// Icon data as `width * height` 8-bit ARGB values.
        data: Vec<u8>,
    },
    /// Icon is stored as raw WebP format.
    WebP {
        /// Raw WebP bytes.
        data: Vec<u8>,
    },
}

impl Default for Icon {
    fn default() -> Self {
        Self::Argb {
            width: 0,
            height: 0,
            data: vec![],
        }
    }
}
