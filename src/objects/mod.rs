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
}
