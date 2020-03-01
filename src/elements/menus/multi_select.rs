use crate::elements::menus::OptionNestingType;
use crate::objects::{OptionInput, Text};

use derive_builder::Builder;
use serde::{Serialize, Serializer};

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
