mod text;
mod option;

pub use crate::objects::text::{FormattingType, Text, TextBuilder};
pub use crate::objects::option::{OptionInput, OptionInputBuilder, OptionInputGroup};

use derive_builder::Builder;
use serde::Serialize;

// Reference: https://api.slack.com/reference/block-kit/composition-objects#text

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct ConfirmationDialog {
    pub title: Text,
    pub text: Text,
    pub confirm: Text,
    pub deny: Text,
}

pub enum Object {
    Text(Text),
    ConfirmationDialog(ConfirmationDialog),
    Option(OptionInput),
    OptionInputGroup(OptionInputGroup),
}
