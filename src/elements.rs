// Reference: https://api.slack.com/reference/block-kit/block-elements

use crate::objects::{ConfirmationDialog, OptionInput, OptionInputGroup, Text};

pub use chrono::{Date, DateTime, Utc};

use url::Url;

pub enum ButtonStyle {
    Default,
    Primary,
    Danger,
}

pub struct Button {
    text: Text,
    action_id: String,
    url: Option<Url>,
    value: Option<String>,
    style: Option<ButtonStyle>,
    confirm: Option<ConfirmationDialog>,
}

pub struct Checkboxes {
    action_id: String,
    Options: Vec<OptionInput>,
    initial_options: Option<Vec<OptionInput>>,
    confirm: Option<ConfirmationDialog>,
}

pub struct DatePicker {
    action_id: String,
    placeholder: Option<Text>,
    // TODO: Should we allow timezones here?
    initial_date: Option<Date<Utc>>,
    confirm: Option<ConfirmationDialog>,
}

pub struct Image {
    url: Url,
    alt_text: String,
}

pub struct OverflowMenu {
    action_id: String,
    options: Vec<OptionInput>,
    confirm: Option<ConfirmationDialog>,
}

pub struct PlainTextInput {
    action_id: String,
    placeholder: Option<Text>,
    initial_value: Option<String>,
    multiline: Option<bool>,
    min_length: Option<u32>,
    max_length: Option<u32>,
}

pub struct RadioButtonGroup {
    action_id: String,
    options: Vec<OptionInput>,
    initial_option: Option<OptionInput>,
    confirm: Option<ConfirmationDialog>,
}
pub struct SelectMenus {
    action_id: String,
    placeholder: Text,
    options: SelectMenuType,
}

pub enum OptionNestingType {
    Flat(Vec<OptionInput>),
    Groups(Vec<OptionInputGroup>),
}

pub struct MultiSelectMenu {
    placeholder: Text,
    action_id: String,
    options: MultiSelectMenuType,
    max_selected_items: u32,
}

pub enum MultiSelectMenuType {
    Static(StaticMultiMenu),
    External(ExternalMultiMenu),
    Users(UserMultiMenu),
    Conversations(ConversationMultiMenu),
    Channels(ChannelMultiMenu),
}

pub struct StaticMultiMenu  {
    options: OptionNestingType,
    initial_options: Option<Vec<OptionInput>>,
}

pub struct ExternalMultiMenu {
    initial_options: Option<Vec<OptionInput>>,
    min_query_length: Option<u32>,
}

pub struct UserMultiMenu {
    initial_users: Option<Vec<String>>,
}

pub struct ConversationMultiMenu {
    initial_conversations: Option<Vec<String>>,
}

pub struct ChannelMultiMenu {
    initial_channels: Option<Vec<String>>,
    options: OptionNestingType,
}

pub enum SelectMenuType {
    Static(StaticMenu),
    External(ExternalMenu),
    Users(UserMenu),
    Conversations(ConversationMenu),
    Channels(ChannelMenu),
}

pub struct StaticMenu  {
    option: OptionNestingType,
    initial_option: Option<OptionInput>,
}

pub struct ExternalMenu {
    initial_option: Option<OptionInput>,
    min_query_length: Option<u32>,
}

pub struct UserMenu {
    initial_user: Option<String>,
}

pub struct ConversationMenu {
    initial_conversation: Option<String>,
}

pub struct ChannelMenu {
    initial_channel: Option<String>,
    option: OptionNestingType,
}
