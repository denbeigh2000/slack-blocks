// Reference: https://api.slack.com/reference/block-kit/block-elements
mod button;
mod datepicker;
mod menus;

pub use crate::elements::button::{Button, ButtonStyle};
pub use crate::elements::datepicker::DatePicker;
pub use crate::elements::menus::*;
use crate::objects::{ConfirmationDialog, OptionInput, Text};

pub use chrono::{Date, Utc};

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use url::Url;

#[derive(Serialize)]
pub struct Checkboxes {
    action_id: String,
    options: Vec<OptionInput>,
    initial_options: Option<Vec<OptionInput>>,
    confirm: Option<ConfirmationDialog>,
}

pub struct CheckboxesBuilder {
    action_id: String,
    options: Vec<OptionInput>,
    initial_options: Option<Vec<OptionInput>>,
    confirm: Option<ConfirmationDialog>,
}

impl CheckboxesBuilder {
    pub fn new(action_id: String, options: Vec<OptionInput>) -> Self {
        Self {
            action_id,
            options,
            initial_options: None,
            confirm: None,
        }
    }

    pub fn set_initial_options(mut self, init_options: Vec<OptionInput>) -> Self {
        self.initial_options = Some(init_options);
        self
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> Checkboxes {
        Checkboxes {
            action_id: self.action_id,
            options: self.options,
            initial_options: self.initial_options,
            confirm: self.confirm,
        }
    }
}

pub struct Image {
    url: Url,
    alt_text: String,
}

impl Image {
    pub fn new(url: Url, alt_text: String) -> Self {
        Self { url, alt_text }
    }
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

#[derive(Serialize)]
pub struct PlainTextInput {
    action_id: String,
    placeholder: Option<Text>,
    initial_value: Option<String>,
    multiline: Option<bool>,
    min_length: Option<u32>,
    max_length: Option<u32>,
}

pub struct PlainTextInputBuilder {
    action_id: String,
    placeholder: Option<Text>,
    initial_value: Option<String>,
    multiline: Option<bool>,
    min_length: Option<u32>,
    max_length: Option<u32>,
}

impl PlainTextInputBuilder {
    pub fn new<S: Into<String>>(action_id: S) -> Self {
        Self {
            action_id: action_id.into(),
            placeholder: None,
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
        }
    }

    pub fn set_placeholder(mut self, ph: Text) -> Self {
        self.placeholder = Some(ph);
        self
    }

    pub fn set_initial_value<S: Into<String>>(mut self, value: S) -> Self {
        self.initial_value = Some(value.into());
        self
    }

    pub fn set_multiline(mut self, ml: bool) -> Self {
        self.multiline = Some(ml);
        self
    }

    pub fn set_min_length(mut self, ml: u32) -> Self {
        self.min_length = Some(ml);
        self
    }

    pub fn set_max_length(mut self, ml: u32) -> Self {
        self.max_length = Some(ml);
        self
    }

    pub fn build(self) -> PlainTextInput {
        PlainTextInput {
            action_id: self.action_id,
            placeholder: self.placeholder,
            initial_value: self.initial_value,
            multiline: self.multiline,
            min_length: self.min_length,
            max_length: self.max_length,
        }
    }
}

#[derive(Serialize)]
pub struct RadioButtonGroup {
    pub action_id: String,
    pub options: Vec<OptionInput>,
    pub initial_option: Option<OptionInput>,
    pub confirm: Option<ConfirmationDialog>,
}

pub struct RadioButtonGroupBuilder {
    action_id: String,
    options: Vec<OptionInput>,
    initial_option: Option<OptionInput>,
    confirm: Option<ConfirmationDialog>,
}

impl RadioButtonGroupBuilder {
    pub fn new<S: Into<String>, O: Into<Vec<OptionInput>>>(action_id: S, options: O) -> Self {
        Self {
            action_id: action_id.into(),
            options: options.into(),
            initial_option: None,
            confirm: None,
        }
    }

    pub fn set_initial_option(mut self, init_option: OptionInput) -> Self {
        self.initial_option = Some(init_option);
        self
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> RadioButtonGroup {
        RadioButtonGroup {
            action_id: self.action_id,
            options: self.options,
            initial_option: self.initial_option,
            confirm: self.confirm,
        }
    }
}
