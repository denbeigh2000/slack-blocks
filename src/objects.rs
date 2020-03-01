// Reference: https://api.slack.com/reference/block-kit/composition-objects#text

use derive_builder::Builder;
pub use url::Url;

pub enum FormattingType {
    PlainText,
    Markdown,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Text {
    pub formatting_type: FormattingType,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct ConfirmationDialog {
    pub title: Text,
    pub text: Text,
    pub confirm: Text,
    pub deny: Text,
}

// TODO: This is only available in overflow menus, is there something we can
// to do make this compile-time safe?
#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct OptionInput {
    pub text: Text,
    pub value: String,
    pub description: Option<Text>,
    pub url: Option<Url>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct OptionInputGroup {
    pub label: Text,
    pub options: Vec<OptionInput>,
}

pub enum Object {
    Text(Text),
    ConfirmationDialog(ConfirmationDialog),
    Option(OptionInput),
    OptionInputGroup(OptionInputGroup),
}
