use crate::elements::*;

use serde::{Serialize, Serializer};

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


