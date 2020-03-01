mod multi_select;
mod select;

pub use multi_select::*;
pub use select::*;

use crate::objects::{ConfirmationDialog, OptionInput, OptionInputGroup};

use derive_builder::Builder;
use serde::{Serialize, Serializer};

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct OverflowMenu {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub confirm: Option<ConfirmationDialog>,
}

pub enum OptionNestingType {
    Flat(Vec<OptionInput>),
    Groups(Vec<OptionInputGroup>),
}

impl Serialize for OptionNestingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OptionNestingType::Flat(e) => e.serialize(serializer),
            OptionNestingType::Groups(e) => e.serialize(serializer),
        }
    }
}
