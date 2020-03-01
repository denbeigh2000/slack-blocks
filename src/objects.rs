// Reference: https://api.slack.com/reference/block-kit/composition-objects#text

pub use url::Url;
use derive_builder::Builder;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

pub enum FormattingType {
    PlainText,
    Markdown,
}

impl Serialize for FormattingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            FormattingType::PlainText => "plain_text",
            FormattingType::Markdown => "mrkdwn",
        })
    }
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct Text {
    pub formatting_type: FormattingType,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
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

impl Serialize for OptionInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut size = 1;
        if self.description.is_some() {
            size += 1;
        }

        if self.url.is_some() {
            size += 1;
        }

        let mut map = serializer.serialize_map(Some(size))?;
        if let Some(p) = &self.description {
            map.serialize_entry("description", &p)?;
        }
        if let Some(u) = &self.url {
            map.serialize_entry("url", u.as_str())?;
        }
        map.end()
    }
}


#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
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
