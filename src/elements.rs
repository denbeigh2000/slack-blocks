// Reference: https://api.slack.com/reference/block-kit/block-elements

use crate::objects::{ConfirmationDialog, OptionInput, OptionInputGroup, Text};

pub use chrono::{Date, DateTime, Utc};
use derive_builder::Builder;
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

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Button {
    pub text: Text,
    pub action_id: String,
    pub url: Option<Url>,
    pub value: Option<String>,
    pub style: Option<ButtonStyle>,
    pub confirm: Option<ConfirmationDialog>,
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

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct Checkboxes {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub initial_options: Option<Vec<OptionInput>>,
    pub confirm: Option<ConfirmationDialog>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct DatePicker {
    pub action_id: String,
    pub placeholder: Option<Text>,
    // TODO: Should we allow timezones here?
    pub initial_date: Option<Date<Utc>>,
    pub confirm: Option<ConfirmationDialog>,
}

impl Serialize for DatePicker {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut size = 1;
        if self.placeholder.is_some() {
            size += 1;
        }

        if self.initial_date.is_some() {
            size += 1;
        }

        if self.confirm.is_some() {
            size += 1;
        }

        let mut map = serializer.serialize_map(Some(size))?;
        map.serialize_entry("action_id", &self.action_id)?;
        if let Some(p) = &self.placeholder {
            map.serialize_entry("placeholder", &p)?;
        }
        if let Some(d) = &self.initial_date {
            map.serialize_entry("initial_date", &d.format("%Y-%M-%d").to_string())?;
        }
        if let Some(c) = &self.confirm {
            map.serialize_entry("confirm", &c)?;
        }
        map.end()
    }
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Image {
    pub url: Url,
    pub alt_text: String,
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("url", self.url.as_str())?;
        map.serialize_entry("alt_text", &self.alt_text)?;
        map.end()
    }
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct OverflowMenu {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub confirm: Option<ConfirmationDialog>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct PlainTextInput {
    pub action_id: String,
    pub placeholder: Option<Text>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct RadioButtonGroup {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub initial_option: Option<OptionInput>,
    pub confirm: Option<ConfirmationDialog>,
}
#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct SelectMenus {
    pub action_id: String,
    pub placeholder: Text,
    pub options: SelectMenuType,
}

pub enum OptionNestingType {
    Flat(Vec<OptionInput>),
    Groups(Vec<OptionInputGroup>),
}

impl Serialize for OptionNestingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OptionNestingType::Flat(e) => e.serialize(serializer),
            OptionNestingType::Groups(e) => e.serialize(serializer),
        }
    }
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct MultiSelectMenu {
    pub placeholder: Text,
    pub action_id: String,
    pub options: MultiSelectMenuType,
    pub max_selected_items: u32,
}

pub enum MultiSelectMenuType {
    Static(StaticMultiMenu),
    External(ExternalMultiMenu),
    Users(UserMultiMenu),
    Conversations(ConversationMultiMenu),
    Channels(ChannelMultiMenu),
}

impl Serialize for MultiSelectMenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MultiSelectMenuType::Static(e) => e.serialize(serializer),
            MultiSelectMenuType::External(e) => e.serialize(serializer),
            MultiSelectMenuType::Users(e) => e.serialize(serializer),
            MultiSelectMenuType::Conversations(e) => e.serialize(serializer),
            MultiSelectMenuType::Channels(e) => e.serialize(serializer),
        }
    }
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct StaticMultiMenu {
    pub options: OptionNestingType,
    pub initial_options: Option<Vec<OptionInput>>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct ExternalMultiMenu {
    pub initial_options: Option<Vec<OptionInput>>,
    pub min_query_length: Option<u32>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct UserMultiMenu {
    pub initial_users: Option<Vec<String>>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct ConversationMultiMenu {
    pub initial_conversations: Option<Vec<String>>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct ChannelMultiMenu {
    pub initial_channels: Option<Vec<String>>,
    pub options: OptionNestingType,
}

pub enum SelectMenuType {
    Static(StaticMenu),
    External(ExternalMenu),
    Users(UserMenu),
    Conversations(ConversationMenu),
    Channels(ChannelMenu),
}

impl Serialize for SelectMenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SelectMenuType::Static(e) => e.serialize(serializer),
            SelectMenuType::External(e) => e.serialize(serializer),
            SelectMenuType::Users(e) => e.serialize(serializer),
            SelectMenuType::Conversations(e) => e.serialize(serializer),
            SelectMenuType::Channels(e) => e.serialize(serializer),
        }
    }
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct StaticMenu {
    pub option: OptionNestingType,
    pub initial_option: Option<OptionInput>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct ExternalMenu {
    pub initial_option: Option<OptionInput>,
    pub min_query_length: Option<u32>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct UserMenu {
    pub initial_user: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct ConversationMenu {
    pub initial_conversation: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct ChannelMenu {
    pub initial_channel: Option<String>,
    pub option: OptionNestingType,
}
