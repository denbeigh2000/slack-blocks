mod text;
mod option;

pub use crate::objects::text::{FormattingType, Text, TextBuilder};
pub use crate::objects::option::{OptionInput, OptionInputBuilder, OptionInputGroup};

use serde::{Serialize, Serializer};

// Reference: https://api.slack.com/reference/block-kit/composition-objects#text

#[derive(Serialize)]
pub struct ConfirmationDialog {
    title: Text,
    text: Text,
    confirm: Text,
    deny: Text,
}

impl ConfirmationDialog {
    pub fn new<S, T, U>(title_text: S, text: Text, confirm_text: T, deny_text: U) -> Self
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        Self {
            title: Text::builder(FormattingType::PlainText, title_text).build(),
            text,
            confirm: Text::builder(FormattingType::PlainText, confirm_text).build(),
            deny: Text::builder(FormattingType::PlainText, deny_text).build(),
        }
    }
}

pub enum Object {
    Text(Text),
    ConfirmationDialog(ConfirmationDialog),
    Option(OptionInput),
    OptionInputGroup(OptionInputGroup),
}

impl Into<Object> for Text {
    fn into(self) -> Object {
        Object::Text(self)
    }
}

impl Into<Object> for ConfirmationDialog {
    fn into(self) -> Object {
        Object::ConfirmationDialog(self)
    }
}

impl Into<Object> for OptionInput {
    fn into(self) -> Object {
        Object::Option(self)
    }
}

impl Into<Object> for OptionInputGroup {
    fn into(self) -> Object {
        Object::OptionInputGroup(self)
    }
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Object::Text(e) => e.serialize(serializer),
            Object::ConfirmationDialog(e) => e.serialize(serializer),
            Object::Option(e) => e.serialize(serializer),
            Object::OptionInputGroup(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn confirmation_dialog_basic() {
        let dialog = ConfirmationDialog::new(
            "Confirm?",
            Text::builder(FormattingType::Markdown, "testing").build(),
            "Yes",
            "No",
        );
        let json = serde_json::to_string(&dialog).unwrap();
        assert_eq!(
            json.as_str(),
            r#"{"title":{"type":"plain_text","text":"Confirm?"},"text":{"type":"mrkdwn","text":"testing"},"confirm":{"type":"plain_text","text":"Yes"},"deny":{"type":"plain_text","text":"No"}}"#
        );
    }

    #[test]
    fn object_serialize() {
        let dialog: Object = ConfirmationDialog::new(
            "Confirm?",
            Text::builder(FormattingType::Markdown, "testing").build(),
            "Yes",
            "No",
        ).into();

        let json = serde_json::to_string(&dialog).unwrap();
        assert_eq!(
            json.as_str(),
            r#"{"title":{"type":"plain_text","text":"Confirm?"},"text":{"type":"mrkdwn","text":"testing"},"confirm":{"type":"plain_text","text":"Yes"},"deny":{"type":"plain_text","text":"No"}}"#
        );
    }
}
