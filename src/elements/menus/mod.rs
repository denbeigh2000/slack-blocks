mod multi_select;
mod overflow;
mod select;

pub use overflow::{OverflowMenu, OverflowMenuBuilder};
pub use multi_select::*;
pub use select::*;

use crate::objects::{OptionInput, OptionInputGroup};
use serde::{Serialize, Serializer};

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
