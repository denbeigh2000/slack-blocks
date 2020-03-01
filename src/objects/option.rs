use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
pub use url::Url;

pub use crate::objects::{FormattingType, Text, TextBuilder};
//
// TODO: This is only available in overflow menus, is there something we can
// to do make this compile-time safe?
pub struct OptionInput {
    text: Text,
    value: String,
    description: Option<Text>,
    url: Option<Url>,
}

impl OptionInput {
    pub fn builder<S: Into<String>>(text: Text, value: S) -> OptionInputBuilder {
        OptionInputBuilder::new(text, value)
    }
}

pub struct OptionInputBuilder {
    text: Text,
    value: String,
    description: Option<Text>,
    url: Option<Url>,
}

impl OptionInputBuilder {
    pub fn new<S: Into<String>>(text: Text, value: S) -> Self {
        Self {
            text,
            value: value.into(),
            description: None,
            url: None,
        }
    }

    pub fn set_description(mut self, desc: Text) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    pub fn build(self) -> OptionInput {
        OptionInput {
            text: self.text,
            value: self.value,
            description: self.description,
            url: self.url,
        }
    }
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

        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("value", &self.value)?;

        if let Some(p) = &self.description {
            map.serialize_entry("description", &p)?;
        }
        if let Some(u) = &self.url {
            map.serialize_entry("url", u.as_str())?;
        }
        map.end()
    }
}

#[derive(Serialize)]
pub struct OptionInputGroup {
    label: Text,
    options: Vec<OptionInput>,
}

impl OptionInputGroup {
    pub fn new(label: Text, options: Vec<OptionInput>) -> Self {
        Self { label, options }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let text = Text::builder(FormattingType::PlainText, "Maru").build();
        let option = OptionInput::builder(text, "maru").build();
        let json = serde_json::to_string(&option).unwrap();
        assert_eq!(
            json.as_str(),
            r#"{"text":{"type":"plain_text","text":"Maru"},"value":"maru"}"#
        );
    }

    #[test]
    fn all() {
        let text = Text::builder(FormattingType::Markdown, "Maru").build();
        let desc = Text::builder(FormattingType::PlainText, "A test option").build();
        let option = OptionInput::builder(text, "maru")
            .set_url(
                "https://slack.example.com/redirect?code=123"
                    .parse()
                    .unwrap(),
            )
            .set_description(desc)
            .build();
        let json = serde_json::to_string(&option).unwrap();
        assert_eq!(
            json.as_str(),
            r#"{"text":{"type":"mrkdwn","text":"Maru"},"value":"maru","description":{"type":"plain_text","text":"A test option"},"url":"https://slack.example.com/redirect?code=123"}"#
        );
    }
}
