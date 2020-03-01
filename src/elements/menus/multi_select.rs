use crate::elements::menus::OptionNestingType;
use crate::objects::{OptionInput, Text};

use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct MultiSelectMenu {
    pub action_id: String,
    pub placeholder: Text,
    pub options: MultiSelectMenuType,
}

impl MultiSelectMenu {
    pub fn new<S: Into<String>>(action_id: S, placeholder: Text, options: MultiSelectMenuType) -> Self {
        Self {
            action_id: action_id.into(),
            placeholder,
            options,
        }
    }
}

pub enum MultiSelectMenuType {
    Static(StaticMultiMenu),
    External(ExternalMultiMenu),
    User(UserMultiMenu),
    Conversation(ConversationMultiMenu),
    Channel(ChannelMultiMenu),
}

impl Serialize for MultiSelectMenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MultiSelectMenuType::Static(e) => e.serialize(serializer),
            MultiSelectMenuType::External(e) => e.serialize(serializer),
            MultiSelectMenuType::User(e) => e.serialize(serializer),
            MultiSelectMenuType::Conversation(e) => e.serialize(serializer),
            MultiSelectMenuType::Channel(e) => e.serialize(serializer),
        }
    }
}

#[derive(Serialize)]
pub struct StaticMultiMenu {
    pub options: OptionNestingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<Vec<OptionInput>>,
}

impl StaticMultiMenu {
    pub fn new(options: OptionNestingType) -> Self {
        Self {
            options,
            initial_option: None,
        }
    }

    pub fn new_with_initial(options: OptionNestingType, init_option: Vec<OptionInput>) -> Self {
        Self {
            options,
            initial_option: Some(init_option),
        }
    }
}

impl Into<MultiSelectMenuType> for StaticMultiMenu {
    fn into(self) -> MultiSelectMenuType {
        MultiSelectMenuType::Static(self)
    }
}

#[derive(Default, Serialize)]
pub struct ExternalMultiMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<Vec<OptionInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_query_length: Option<Vec<u32>>,
}

impl Into<MultiSelectMenuType> for ExternalMultiMenu {
    fn into(self) -> MultiSelectMenuType {
        MultiSelectMenuType::External(self)
    }
}

#[derive(Default)]
pub struct ExternalMultiMenuBuilder {
    initial_option: Option<Vec<OptionInput>>,
    min_query_length: Option<Vec<u32>>,
}

impl ExternalMultiMenuBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_initial_option(mut self, option: Vec<OptionInput>) -> Self{
        self.initial_option = Some(option);
        self
    }

    pub fn set_query_length(mut self, min_len: Vec<u32>) -> Self {
        self.min_query_length = Some(min_len);
        self
    }

    pub fn build(self) -> ExternalMultiMenu {
        ExternalMultiMenu {
            initial_option: self.initial_option,
            min_query_length: self.min_query_length,
        }
    }
}

#[derive(Default, Serialize)]
pub struct UserMultiMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_user: Option<Vec<String>>,
}

impl Into<MultiSelectMenuType> for UserMultiMenu {
    fn into(self) -> MultiSelectMenuType {
        MultiSelectMenuType::User(self)
    }
}

impl UserMultiMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_initial<S: Into<Vec<String>>>(init_user: S) -> Self {
        Self {
            initial_user: Some(init_user.into()),
        }
    }
}

#[derive(Default, Serialize)]
pub struct ConversationMultiMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_conversation: Option<Vec<String>>,
}

impl Into<MultiSelectMenuType> for ConversationMultiMenu {
    fn into(self) -> MultiSelectMenuType {
        MultiSelectMenuType::Conversation(self)
    }
}

impl ConversationMultiMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_initial<S: Into<Vec<String>>>(init_conversation: S) -> Self {
        Self {
            initial_conversation: Some(init_conversation.into()),
        }
    }
}

#[derive(Serialize)]
pub struct ChannelMultiMenu {
    options: OptionNestingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_channel: Option<Vec<String>>,
}

impl Into<MultiSelectMenuType> for ChannelMultiMenu {
    fn into(self) -> MultiSelectMenuType {
        MultiSelectMenuType::Channel(self)
    }
}

impl ChannelMultiMenu {
    pub fn new(options: OptionNestingType) -> Self {
        Self {
            options,
            initial_channel: None,
        }
    }

    pub fn new_with_initial<S: Into<Vec<String>>>(options: OptionNestingType, init_channel: S) -> Self {
        Self {
            options,
            initial_channel: Some(init_channel.into()),
        }
    }
}
