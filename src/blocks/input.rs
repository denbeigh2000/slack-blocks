use crate::elements::*;
use crate::objects::Text;

use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Input {
    label: Text,
    element: InputElement,
    block_id: Option<String>,
    // This must be plain-text, is there any way to enforce this at
    // compile-time?
    hint: Option<Text>,
    optional: Option<bool>,
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

