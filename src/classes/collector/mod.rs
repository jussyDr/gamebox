//! TODO

use std::rc::Rc;

mod read;

#[derive(Default)]
/// TODO
pub struct Collector {
    name: String,
    icon_width: u16,
    icon_height: u16,
    icon_data: Vec<u8>,
    collection: Option<Rc<str>>,
    description: String,
}

impl Collector {
    /// Name of this collector.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Pixel width of this collector's icon.
    pub fn icon_width(&self) -> u16 {
        self.icon_width
    }

    /// Pixel height of this collector's icon.
    pub fn icon_height(&self) -> u16 {
        self.icon_height
    }

    /// Icon data of this collector as `icon_width * icon_height` 8-bit ARGB values.
    pub fn icon_data(&self) -> &[u8] {
        &self.icon_data
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
