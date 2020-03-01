use crate::elements::*;
use crate::objects::Text;

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use url::Url;

#[derive(Serialize)]
pub struct Actions {
    elements: Vec<ActionsElement>,
    block_id: Option<String>,
}

impl Actions {
    pub fn new(elements: Vec<ActionsElement>) -> Self {
        Actions {
            elements,
            block_id: None,
        }
    }

    pub fn new_with_id<S: Into<String>>(block_id: S, elements: Vec<ActionsElement>) -> Self {
        Self {
            elements,
            block_id: Some(block_id.into()),
        }
    }
}

#[derive(Serialize)]
pub struct Context {
    pub elements: Vec<ContextElement>,
    pub block_id: Option<String>,
}

impl Context {
    pub fn new(elements: Vec<ContextElement>) -> Self {
        Context {
            elements,
            block_id: None,
        }
    }

    pub fn new_with_id<S: Into<String>>(block_id: S, elements: Vec<ContextElement>) -> Self {
        Self {
            elements,
            block_id: Some(block_id.into()),
        }
    }
}

#[derive(Default, Serialize)]
pub struct Divider {
    pub block_id: Option<String>,
}

impl Divider {
    pub fn new() -> Self {
        Divider::default()
    }

    pub fn new_with_id<S: Into<String>>(block_id: S) -> Self {
        Self {
            block_id: Some(block_id.into()),
        }
    }
}

#[derive(Serialize)]
pub struct File {
    external_id: String,
    block_id: Option<String>,
}

impl File {
    pub fn new<S: Into<String>>(external_id: S) -> Self {
        File {
            external_id: external_id.into(),
            block_id: None,
        }
    }

    pub fn new_with_id<S: Into<String>, T: Into<String>>(block_id: S, external_id: T) -> Self {
        Self {
            external_id: external_id.into(),
            block_id: Some(block_id.into()),
        }
    }
}

pub struct Image {
    image_url: Url,
    alt_text: String,
    title: Option<Text>,
    block_id: Option<String>,
}

pub struct ImageBuilder {
    image_url: Url,
    alt_text: String,
    title: Option<Text>,
    block_id: Option<String>,
}

impl ImageBuilder {
    pub fn new<S: Into<String>>(url: Url, alt_text: S) -> ImageBuilder {
        ImageBuilder {
            image_url: url,
            alt_text: alt_text.into(),
            title: None,
            block_id: None,
        }
    }

    pub fn set_title(mut self, title: Text) -> Self {
        self.title = Some(title);
        self
    }

    pub fn set_block_id(mut self, block_id: String) -> Self {
        self.block_id = Some(block_id);
        self
    }

    pub fn build(self) -> Image {
        Image {
            image_url: self.image_url,
            alt_text: self.alt_text,
            title: self.title,
            block_id: self.block_id,
        }
    }
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

#[derive(Serialize)]
pub struct Input {
    pub label: Text,
    pub element: InputElement,
    pub block_id: Option<String>,
    // This must be plain-text, is there any way to enforce this at
    // compile-time?
    pub hint: Option<Text>,
    pub optional: Option<bool>,
}

pub struct InputBuilder {
    label: Text,
    element: InputElement,
    block_id: Option<String>,
    hint: Option<Text>,
    optional: Option<bool>,
}

impl InputBuilder {
    pub fn new(label: Text, element: InputElement) -> Self {
        Self {
            label,
            element,
            block_id: None,
            hint: None,
            optional: None,
        }
    }

    pub fn set_block_id(mut self, block_id: String) -> Self {
        self.block_id = Some(block_id);
        self
    }

    pub fn set_hint(mut self, hint: Text) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn set_optional(mut self, optional: bool) -> Self {
        self.optional = Some(optional);
        self
    }

    pub fn build(self) -> Input {
        Input {
            label: self.label,
            element: self.element,
            block_id: self.block_id,
            hint: self.hint,
            optional: self.optional,
        }
    }
}

#[derive(Serialize)]
pub struct Section {
    pub text: Text,
    pub block_id: Option<String>,
    pub fields: Option<Vec<Text>>,
    pub accessory: Option<SectionElement>,
}

pub struct SectionBuilder {
    text: Text,
    block_id: Option<String>,
    fields: Option<Vec<Text>>,
    accessory: Option<SectionElement>,
}

impl SectionBuilder {
    pub fn new(text: Text) -> Self {
        Self {
            text,
            block_id: None,
            fields: None,
            accessory: None,
        }
    }

    pub fn set_block_id(mut self, block_id: String) -> Self {
        self.block_id = Some(block_id);
        self
    }

    pub fn set_fields(mut self, fields: Vec<Text>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn set_accessory(mut self, accessory: SectionElement) -> Self {
        self.accessory = Some(accessory);
        self
    }

    pub fn build(self) -> Section {
        Section {
            text: self.text,
            block_id: self.block_id,
            fields: self.fields,
            accessory: self.accessory,
        }
    }
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
