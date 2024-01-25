//! Types used for reading and writing [Item] nodes.

mod read;
mod write;

use crate::common::{Class, ClassId, EngineId};

use super::{collector::Collector, static_object_model::Solid2Model};

/// Node type corresponding to GameBox files with the extension `Item.Gbx`.
#[derive(Default)]
pub struct Item {
    parent: Collector,
}

impl Class for Item {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 2);
}

#[derive(Default, Clone)]
struct ItemEntityModel {
    solid_to_model: Solid2Model,
}

impl Class for ItemEntityModel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 39);
}

struct ItemEntityModelEdition;

impl Class for ItemEntityModelEdition {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 38);
}

impl Default for ItemEntityModelEdition {
    fn default() -> Self {
        Self
    }
}

struct TreeGenerator;

struct Crystal {
    parent: TreeGenerator,
}

impl Class for Crystal {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 3);
}

impl Default for Crystal {
    fn default() -> Self {
        Self {
            parent: TreeGenerator,
        }
    }
}

struct ItemPlacementParam;

impl Class for ItemPlacementParam {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 32);
}

impl Default for ItemPlacementParam {
    fn default() -> Self {
        Self
    }
}

#[derive(Default)]
struct ItemPlacement;

impl Class for ItemPlacement {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 391);
}
