use crate::elements::*;
use crate::objects::Text;

use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Section {
    text: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<Text>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessory: Option<SectionElement>,
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


