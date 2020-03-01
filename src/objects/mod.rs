mod text;
mod option;

pub use crate::objects::text::{FormattingType, Text, TextBuilder};
pub use crate::objects::option::{OptionInput, OptionInputBuilder, OptionInputGroup};

use serde::Serialize;

// Reference: https://api.slack.com/reference/block-kit/composition-objects#text

#[derive(Serialize)]
pub struct ConfirmationDialog {
    title: Text,
    text: Text,
    confirm: Text,
    deny: Text,
}

impl ConfirmationDialog {
    pub fn new(title: Text, text: Text, confirm: Text, deny: Text) -> Self {
        Self {
            title,
            text,
            confirm,
            deny,
        }
    }
}

pub enum Object {
    Text(Text),
    ConfirmationDialog(ConfirmationDialog),
    Option(OptionInput),
    OptionInputGroup(OptionInputGroup),
}
