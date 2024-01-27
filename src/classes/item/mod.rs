//! Types used for reading and writing [Item] nodes.

mod read;
mod write;

use std::rc::Rc;

use crate::common::{Class, ClassId, EngineId};

use super::{collector::Collector, crystal::Crystal, static_object_model::Solid2Model};

/// Node type corresponding to GameBox files with the extension `Item.Gbx`.
#[derive(Default)]
pub struct Item {
    parent: Collector,
    model: ItemModel,
}

impl Class for Item {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 2);
}

enum ItemModel {
    Block(BlockItem),
    Edition(ItemEntityModelEdition),
    Normal(ItemEntityModel),
    VariantList,
}

impl Default for ItemModel {
    fn default() -> Self {
        Self::Edition(ItemEntityModelEdition)
    }
}

#[derive(Default)]
struct BlockItem {
    variants: Vec<Option<Rc<Crystal>>>,
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
