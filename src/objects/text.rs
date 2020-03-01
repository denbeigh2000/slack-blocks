use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Text {
    #[serde(rename(serialize = "type"))]
    formatting_type: FormattingType,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl Text {
    pub fn builder<S: Into<String>>(formatting_type: FormattingType, text: S) -> TextBuilder {
        TextBuilder::new(formatting_type, text)
    }
}

pub struct TextBuilder {
    formatting_type: FormattingType,
    text: String,
    emoji: Option<bool>,
    verbatim: Option<bool>,
}

impl TextBuilder {
    pub fn new<S: Into<String>>(formatting_type: FormattingType, text: S) -> Self {
        Self {
            formatting_type,
            text: text.into(),
            emoji: None,
            verbatim: None,
        }
    }

    pub fn set_emoji(mut self, v: bool) -> Self {
        self.emoji = Some(v);
        self
    }

    pub fn set_verbatim(mut self, v: bool) -> Self {
        self.verbatim = Some(v);
        self
    }

    pub fn build(self) -> Text {
        Text {
            formatting_type: self.formatting_type,
            text: self.text,
            emoji: self.emoji,
            verbatim: self.verbatim,
        }
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_plain() {
        let text = TextBuilder::new(FormattingType::PlainText, "hello, world").build();
        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(
            json.as_str(),
            r#"{"type":"plain_text","text":"hello, world"}"#
        );
    }

    #[test]
    fn text_all() {
        let text = TextBuilder::new(FormattingType::Markdown, "hello, world")
            .set_emoji(true)
            .set_verbatim(false)
            .build();
        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(
            json.as_str(),
            r#"{"type":"mrkdwn","text":"hello, world","emoji":true,"verbatim":false}"#
        );
    }
}
