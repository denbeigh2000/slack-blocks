use crate::elements::menus::OptionNestingType;
use crate::objects::{OptionInput, Text};

use derive_builder::Builder;
use serde::{Serialize, Serializer};

#[builder(setter(into), pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct SelectMenu {
    pub action_id: String,
    pub placeholder: Text,
    pub options: SelectMenuType,
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
