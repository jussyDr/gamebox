//! Types used for reading and writing [Item] nodes.

mod read;
mod write;

use std::rc::Rc;

use crate::{
    common::{Class, ClassId, EngineId},
    engines::plug::{crystal::Crystal, static_object_model::StaticObjectModel},
};

use super::collector::Collector;

/// Node type corresponding to GameBox files with the extension `Item.Gbx` or `Block.Gbx`.
#[derive(Default, Debug)]
pub struct Item {
    parent: Collector,
    model: ItemModel,
}

impl Item {
    /// Model of the item.
    pub fn model(&self) -> &ItemModel {
        &self.model
    }
}

impl Class for Item {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 2);
}

/// Item model.
#[derive(Debug)]
pub enum ItemModel {
    /// Block model.
    Block(BlockItem),
    Entity(ItemEntityModel),
    EntityEdition(ItemEntityModelEdition),
    VariantList,
}

impl Default for ItemModel {
    fn default() -> Self {
        Self::EntityEdition(ItemEntityModelEdition::default())
    }
}

#[derive(Default, Debug)]
struct BlockItem {
    variants: Vec<Option<Rc<Crystal>>>,
}

#[derive(Default, Debug)]
pub struct ItemEntityModel {
    static_object_model: StaticObjectModel,
}

impl ItemEntityModel {
    pub fn static_object_model(&self) -> &StaticObjectModel {
        &self.static_object_model
    }
}

impl Class for ItemEntityModel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 39);
}

#[derive(Default, Debug)]
pub struct ItemEntityModelEdition {
    crystal: Crystal,
}

impl ItemEntityModelEdition {
    pub fn crystal(&self) -> &Crystal {
        &self.crystal
    }
}

impl Class for ItemEntityModelEdition {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 38);
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
