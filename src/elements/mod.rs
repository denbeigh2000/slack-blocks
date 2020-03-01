// Reference: https://api.slack.com/reference/block-kit/block-elements
mod button;
mod datepicker;
mod menus;

pub use crate::elements::button::{Button, ButtonStyle};
pub use crate::elements::datepicker::{DatePicker};
pub use crate::elements::menus::*;
use crate::objects::{ConfirmationDialog, OptionInput, Text};

pub use chrono::{Date, Utc};

use derive_builder::Builder;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use url::Url;

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
