use crate::objects::Text;
use crate::elements::*;

use derive_builder::Builder;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use url::Url;

#[builder(pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct Actions {
    pub elements: Vec<ActionsElement>,
    pub block_id: Option<String>,
}

#[builder(pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct Context {
    pub elements: Vec<ContextElement>,
    pub block_id: Option<String>,
}

#[builder(pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct Divider {
    pub block_id: Option<String>,
}

#[builder(pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct File {
    pub external_id: String,
    pub block_id: Option<String>,
}

#[builder(pattern = "owned")]
#[derive(Builder)]
pub struct Image {
    pub image_url: Url,
    pub alt_text: String,
    pub title: Option<Text>,
    pub block_id: Option<String>,
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut size = 2;
        if self.title.is_some() {
            size += 1;
        }
        if self.block_id.is_some() {
            size += 2;
        }

        let mut map = serializer.serialize_map(Some(size))?;
        map.serialize_entry("image_url", self.image_url.as_str())?;
        map.serialize_entry("alt_text", &self.alt_text)?;
        if let Some(t) = &self.title {
            map.serialize_entry("title", &t)?;
        }
        if let Some(id) = &self.block_id {
            map.serialize_entry("block_id", &id)?;
        }
        map.end()
    }
}

#[builder(pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct Input {
    pub label: Text,
    pub element: InputElement,
    pub block_id: Option<String>,
    // This must be plain-text, is there any way to enforce this at
    // compile-time?
    pub hint: Option<Text>,
    pub optional: Option<bool>,
}

#[builder(pattern = "owned")]
#[derive(Builder, Serialize)]
pub struct Section {
    pub text: Text,
    pub block_id: Option<String>,
    pub fields: Option<Vec<Text>>,
    pub accessory: Option<SectionElement>,
}

impl Into<SectionElement> for Button {
    fn into(self) -> SectionElement {
        SectionElement::Button(self)
    }
}

impl Into<SectionElement> for Checkboxes {
    fn into(self) -> SectionElement {
        SectionElement::Checkboxes(self)
    }
}

impl Into<SectionElement> for DatePicker {
    fn into(self) -> SectionElement {
        SectionElement::DatePicker(self)
    }
}

impl Into<SectionElement> for Image {
    fn into(self) -> SectionElement {
        SectionElement::Image(self)
    }
}

impl Into<SectionElement> for MultiSelectMenu {
    fn into(self) -> SectionElement {
        SectionElement::MultiSelectMenu(self)
    }
}

impl Into<SectionElement> for OverflowMenu {
    fn into(self) -> SectionElement {
        SectionElement::OverflowMenu(self)
    }
}

impl Into<SectionElement> for PlainTextInput {
    fn into(self) -> SectionElement {
        SectionElement::PlainTextInput(self)
    }
}

impl Into<SectionElement> for RadioButtonGroup {
    fn into(self) -> SectionElement {
        SectionElement::RadioButtonGroup(self)
    }
}

impl Into<SectionElement> for SelectMenu {
    fn into(self) -> SectionElement {
        SectionElement::SelectMenu(self)
    }
}

pub enum SectionElement {
    Button(Button),
    Checkboxes(Checkboxes),
    DatePicker(DatePicker),
    Image(Image),
    MultiSelectMenu(MultiSelectMenu),
    OverflowMenu(OverflowMenu),
    PlainTextInput(PlainTextInput),
    RadioButtonGroup(RadioButtonGroup),
    SelectMenu(SelectMenu),
}

impl Serialize for SectionElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SectionElement::Button(e) => e.serialize(serializer),
            SectionElement::Checkboxes(e) => e.serialize(serializer),
            SectionElement::DatePicker(e) => e.serialize(serializer),
            SectionElement::Image(e) => e.serialize(serializer),
            SectionElement::MultiSelectMenu(e) => e.serialize(serializer),
            SectionElement::OverflowMenu(e) => e.serialize(serializer),
            SectionElement::PlainTextInput(e) => e.serialize(serializer),
            SectionElement::RadioButtonGroup(e) => e.serialize(serializer),
            SectionElement::SelectMenu(e) => e.serialize(serializer),
        }
    }
}

impl Into<ActionsElement> for Button {
    fn into(self) -> ActionsElement {
        ActionsElement::Button(self)
    }
}

impl Into<ActionsElement> for Checkboxes {
    fn into(self) -> ActionsElement {
        ActionsElement::Checkboxes(self)
    }
}

impl Into<ActionsElement> for DatePicker {
    fn into(self) -> ActionsElement {
        ActionsElement::DatePicker(self)
    }
}

impl Into<ActionsElement> for OverflowMenu {
    fn into(self) -> ActionsElement {
        ActionsElement::OverflowMenu(self)
    }
}

impl Into<ActionsElement> for PlainTextInput {
    fn into(self) -> ActionsElement {
        ActionsElement::PlainTextInput(self)
    }
}

impl Into<ActionsElement> for RadioButtonGroup {
    fn into(self) -> ActionsElement {
        ActionsElement::RadioButtonGroup(self)
    }
}

impl Into<ActionsElement> for SelectMenu {
    fn into(self) -> ActionsElement {
        ActionsElement::SelectMenu(self)
    }
}

pub enum ActionsElement {
    Button(Button),
    Checkboxes(Checkboxes),
    DatePicker(DatePicker),
    OverflowMenu(OverflowMenu),
    PlainTextInput(PlainTextInput),
    RadioButtonGroup(RadioButtonGroup),
    SelectMenu(SelectMenu),
}

impl Serialize for ActionsElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ActionsElement::Button(e) => e.serialize(serializer),
            ActionsElement::Checkboxes(e) => e.serialize(serializer),
            ActionsElement::DatePicker(e) => e.serialize(serializer),
            ActionsElement::OverflowMenu(e) => e.serialize(serializer),
            ActionsElement::PlainTextInput(e) => e.serialize(serializer),
            ActionsElement::RadioButtonGroup(e) => e.serialize(serializer),
            ActionsElement::SelectMenu(e) => e.serialize(serializer),
        }
    }
}

impl Into<InputElement> for Checkboxes {
    fn into(self) -> InputElement {
        InputElement::Checkboxes(self)
    }
}

impl Into<InputElement> for DatePicker {
    fn into(self) -> InputElement {
        InputElement::DatePicker(self)
    }
}

impl Into<InputElement> for MultiSelectMenu {
    fn into(self) -> InputElement {
        InputElement::MultiSelectMenu(self)
    }
}

impl Into<InputElement> for PlainTextInput {
    fn into(self) -> InputElement {
        InputElement::PlainTextInput(self)
    }
}

impl Into<InputElement> for RadioButtonGroup {
    fn into(self) -> InputElement {
        InputElement::RadioButtonGroup(self)
    }
}

impl Into<InputElement> for SelectMenu {
    fn into(self) -> InputElement {
        InputElement::SelectMenu(self)
    }
}

pub enum InputElement {
    Checkboxes(Checkboxes),
    DatePicker(DatePicker),
    MultiSelectMenu(MultiSelectMenu),
    PlainTextInput(PlainTextInput),
    RadioButtonGroup(RadioButtonGroup),
    SelectMenu(SelectMenu),
}

impl Serialize for InputElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InputElement::Checkboxes(e) => e.serialize(serializer),
            InputElement::DatePicker(e) => e.serialize(serializer),
            InputElement::MultiSelectMenu(e) => e.serialize(serializer),
            InputElement::PlainTextInput(e) => e.serialize(serializer),
            InputElement::RadioButtonGroup(e) => e.serialize(serializer),
            InputElement::SelectMenu(e) => e.serialize(serializer),
        }
    }
}


impl Into<ContextElement> for Image {
    fn into(self) -> ContextElement {
        ContextElement::Image(self)
    }
}

impl Into<ContextElement> for Text {
    fn into(self) -> ContextElement {
        ContextElement::Text(self)
    }
}

pub enum ContextElement {
    Image(Image),
    Text(Text),
}

impl Serialize for ContextElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ContextElement::Image(e) => e.serialize(serializer),
            ContextElement::Text(e) => e.serialize(serializer),
        }
    }
}
