use crate::elements::menus::OptionNestingType;
use crate::objects::{OptionInput, Text};

use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct SelectMultiMenu {
    pub action_id: String,
    pub placeholder: Text,
    pub options: SelectMultiMenuType,
}

impl SelectMultiMenu {
    pub fn new<S: Into<String>>(action_id: S, placeholder: Text, options: SelectMultiMenuType) -> Self {
        Self {
            action_id: action_id.into(),
            placeholder,
            options,
        }
    }
}

pub enum SelectMultiMenuType {
    Static(StaticMultiMenu),
    External(ExternalMultiMenu),
    User(UserMultiMenu),
    Conversation(ConversationMultiMenu),
    Channel(ChannelMultiMenu),
}

impl Serialize for SelectMultiMenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SelectMultiMenuType::Static(e) => e.serialize(serializer),
            SelectMultiMenuType::External(e) => e.serialize(serializer),
            SelectMultiMenuType::User(e) => e.serialize(serializer),
            SelectMultiMenuType::Conversation(e) => e.serialize(serializer),
            SelectMultiMenuType::Channel(e) => e.serialize(serializer),
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

impl Into<SelectMultiMenuType> for StaticMultiMenu {
    fn into(self) -> SelectMultiMenuType {
        SelectMultiMenuType::Static(self)
    }
}

#[derive(Default, Serialize)]
pub struct ExternalMultiMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<Vec<OptionInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_query_length: Option<Vec<u32>>,
}

impl Into<SelectMultiMenuType> for ExternalMultiMenu {
    fn into(self) -> SelectMultiMenuType {
        SelectMultiMenuType::External(self)
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

impl Into<SelectMultiMenuType> for UserMultiMenu {
    fn into(self) -> SelectMultiMenuType {
        SelectMultiMenuType::User(self)
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

impl Into<SelectMultiMenuType> for ConversationMultiMenu {
    fn into(self) -> SelectMultiMenuType {
        SelectMultiMenuType::Conversation(self)
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

impl Into<SelectMultiMenuType> for ChannelMultiMenu {
    fn into(self) -> SelectMultiMenuType {
        SelectMultiMenuType::Channel(self)
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
