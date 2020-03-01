use crate::objects::{ConfirmationDialog, Text};

pub use chrono::{Date, DateTime, Utc};

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use url::Url;

pub enum ButtonStyle {
    Danger,
    Default,
    Primary,
}

impl Serialize for ButtonStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            ButtonStyle::Danger => "danger",
            ButtonStyle::Default => "default",
            ButtonStyle::Primary => "primary",
        })
    }
}

pub struct Button {
    pub text: Text,
    pub action_id: String,
    pub url: Option<Url>,
    pub value: Option<String>,
    pub style: Option<ButtonStyle>,
    pub confirm: Option<ConfirmationDialog>,
}

pub struct ButtonBuilder {
    text: Text,
    action_id: String,
    url: Option<Url>,
    value: Option<String>,
    style: Option<ButtonStyle>,
    confirm: Option<ConfirmationDialog>,
}

impl ButtonBuilder {
    pub fn new<S: Into<String>>(action_id: S, text: Text) -> Self {
        Self {
            action_id: action_id.into(),
            text,
            url: None,
            value: None,
            style: None,
            confirm: None,
        }
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    pub fn set_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn set_style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> Button {
        Button {
            text: self.text,
            action_id: self.action_id,
            url: self.url,
            value: self.value,
            style: self.style,
            confirm: self.confirm,
        }
    }
}

impl Serialize for Button {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut size = 2;
        if self.url.is_some() {
            size += 1;
        }

        if self.value.is_some() {
            size += 1;
        }

        if self.style.is_some() {
            size += 1;
        }

        if self.confirm.is_some() {
            size += 1;
        }
        let mut map = serializer.serialize_map(Some(size))?;

        map.serialize_entry("action_id", &self.action_id)?;
        if let Some(u) = &self.url {
            map.serialize_entry("url", u.as_str())?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", &v)?;
        }
        if let Some(s) = &self.style {
            map.serialize_entry("style", &s)?;
        }
        if let Some(c) = &self.confirm {
            map.serialize_entry("confirm", &c)?;
        }
        map.end()
    }
}


