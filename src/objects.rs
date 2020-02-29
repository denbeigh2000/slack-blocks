// Reference: https://api.slack.com/reference/block-kit/composition-objects#text

pub use url::Url;

pub enum FormattingType {
    PlainText,
    Markdown,
}

pub struct Text {
    formatting_type: FormattingType,
    text: String,
    emoji: Option<bool>,
    verbatim: Option<bool>,
}

pub struct ConfirmationDialog {
    title: Text,
    text: Text,
    confirm: Text,
    deny: Text,
}

// TODO: This is only available in overflow menus, is there something we can
// to do make this compile-time safe?
pub struct OptionInput {
    text: Text,
    value: String,
    description: Option<Text>,
    url: Option<Url>,
}

pub struct OptionInputGroup {
    label: Text,
    options: Vec<OptionInput>,
}

pub enum Object {
    Text(Text),
    ConfirmationDialog(ConfirmationDialog),
    Option(OptionInput),
    OptionInputGroup(OptionInputGroup),
}
