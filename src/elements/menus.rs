use crate::objects::{ConfirmationDialog, OptionInput, OptionInputGroup, Text};

use derive_builder::Builder;
use serde::{Serialize, Serializer};


#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct OverflowMenu {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub confirm: Option<ConfirmationDialog>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct SelectMenus {
    pub action_id: String,
    pub placeholder: Text,
    pub options: SelectMenuType,
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
