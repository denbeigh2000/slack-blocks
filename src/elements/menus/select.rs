use crate::elements::menus::OptionNestingType;
use crate::objects::{OptionInput, Text};

use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct SelectMenu {
    action_id: String,
    placeholder: Text,
    options: SelectMenuType,
}

impl SelectMenu {
    pub fn new<S: Into<String>>(action_id: S, placeholder: Text, options: SelectMenuType) -> Self {
        Self {
            action_id: action_id.into(),
            placeholder,
            options,
        }
    }
}

pub enum SelectMenuType {
    Static(StaticMenu),
    External(ExternalMenu),
    User(UserMenu),
    Conversation(ConversationMenu),
    Channel(ChannelMenu),
}

impl Serialize for SelectMenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SelectMenuType::Static(e) => e.serialize(serializer),
            SelectMenuType::External(e) => e.serialize(serializer),
            SelectMenuType::User(e) => e.serialize(serializer),
            SelectMenuType::Conversation(e) => e.serialize(serializer),
            SelectMenuType::Channel(e) => e.serialize(serializer),
        }
    }
}

#[derive(Serialize)]
pub struct StaticMenu {
    options: OptionNestingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<OptionInput>,
}

impl StaticMenu {
    pub fn new(options: OptionNestingType) -> Self {
        Self {
            options,
            initial_option: None,
        }
    }

    pub fn new_with_initial(options: OptionNestingType, init_option: OptionInput) -> Self {
        Self {
            options,
            initial_option: Some(init_option),
        }
    }
}

impl Into<SelectMenuType> for StaticMenu {
    fn into(self) -> SelectMenuType {
        SelectMenuType::Static(self)
    }
}

#[derive(Default, Serialize)]
pub struct ExternalMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<OptionInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_query_length: Option<u32>,
}

impl Into<SelectMenuType> for ExternalMenu {
    fn into(self) -> SelectMenuType {
        SelectMenuType::External(self)
    }
}

#[derive(Default)]
pub struct ExternalMenuBuilder {
    initial_option: Option<OptionInput>,
    min_query_length: Option<u32>,
}

impl ExternalMenuBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_initial_option(mut self, option: OptionInput) -> Self{
        self.initial_option = Some(option);
        self
    }

    pub fn set_query_length(mut self, min_len: u32) -> Self {
        self.min_query_length = Some(min_len);
        self
    }

    pub fn build(self) -> ExternalMenu {
        ExternalMenu {
            initial_option: self.initial_option,
            min_query_length: self.min_query_length,
        }
    }
}

#[derive(Default, Serialize)]
pub struct UserMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_user: Option<String>,
}

impl Into<SelectMenuType> for UserMenu {
    fn into(self) -> SelectMenuType {
        SelectMenuType::User(self)
    }
}

impl UserMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_initial<S: Into<String>>(init_user: S) -> Self {
        Self {
            initial_user: Some(init_user.into()),
        }
    }
}

#[derive(Default, Serialize)]
pub struct ConversationMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_conversation: Option<String>,
}

impl Into<SelectMenuType> for ConversationMenu {
    fn into(self) -> SelectMenuType {
        SelectMenuType::Conversation(self)
    }
}

impl ConversationMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_initial<S: Into<String>>(init_conversation: S) -> Self {
        Self {
            initial_conversation: Some(init_conversation.into()),
        }
    }
}

#[derive(Serialize)]
pub struct ChannelMenu {
    options: OptionNestingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_channel: Option<String>,
}

impl Into<SelectMenuType> for ChannelMenu {
    fn into(self) -> SelectMenuType {
        SelectMenuType::Channel(self)
    }
}

impl ChannelMenu {
    pub fn new(options: OptionNestingType) -> Self {
        Self {
            options,
            initial_channel: None,
        }
    }

    pub fn new_with_initial<S: Into<String>>(options: OptionNestingType, init_channel: S) -> Self {
        Self {
            options,
            initial_channel: Some(init_channel.into()),
        }
    }
}
