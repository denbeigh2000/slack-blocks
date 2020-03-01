// Reference: https://api.slack.com/reference/block-kit/block-elements

use crate::objects::{ConfirmationDialog, OptionInput, OptionInputGroup, Text};

pub use chrono::{Date, DateTime, Utc};
use derive_builder::Builder;
use url::Url;

pub enum ButtonStyle {
    Default,
    Primary,
    Danger,
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

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
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

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Image {
    pub url: Url,
    pub alt_text: String,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct OverflowMenu {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub confirm: Option<ConfirmationDialog>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct PlainTextInput {
    pub action_id: String,
    pub placeholder: Option<Text>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct RadioButtonGroup {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub initial_option: Option<OptionInput>,
    pub confirm: Option<ConfirmationDialog>,
}
#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct SelectMenus {
    pub action_id: String,
    pub placeholder: Text,
    pub options: SelectMenuType,
}

pub enum OptionNestingType {
    Flat(Vec<OptionInput>),
    Groups(Vec<OptionInputGroup>),
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
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

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct StaticMultiMenu {
    pub options: OptionNestingType,
    pub initial_options: Option<Vec<OptionInput>>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct ExternalMultiMenu {
    pub initial_options: Option<Vec<OptionInput>>,
    pub min_query_length: Option<u32>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct UserMultiMenu {
    pub initial_users: Option<Vec<String>>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct ConversationMultiMenu {
    pub initial_conversations: Option<Vec<String>>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
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

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct StaticMenu {
    pub option: OptionNestingType,
    pub initial_option: Option<OptionInput>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct ExternalMenu {
    pub initial_option: Option<OptionInput>,
    pub min_query_length: Option<u32>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct UserMenu {
    pub initial_user: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct ConversationMenu {
    pub initial_conversation: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct ChannelMenu {
    pub initial_channel: Option<String>,
    pub option: OptionNestingType,
}
