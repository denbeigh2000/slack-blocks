use crate::objects::Text;
use crate::elements::*;

use derive_builder::Builder;
use url::Url;

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Actions {
    pub elements: Vec<ActionsElement>,
    pub block_id: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Context {
    pub elements: Vec<ContextElement>,
    pub block_id: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Divider {
    pub block_id: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct File {
    pub external_id: String,
    pub block_id: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Image {
    pub image_url: Url,
    pub alt_text: String,
    pub title: Option<Text>,
    pub block_id: Option<String>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Input {
    pub label: Text,
    pub element: InputElement,
    pub block_id: Option<String>,
    // This must be plain-text, is there any way to enforce this at
    // compile-time?
    pub hint: Option<Text>,
    pub optional: Option<bool>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
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

impl Into<SectionElement> for SelectMenus {
    fn into(self) -> SectionElement {
        SectionElement::SelectMenus(self)
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
    SelectMenus(SelectMenus),
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

impl Into<ActionsElement> for SelectMenus {
    fn into(self) -> ActionsElement {
        ActionsElement::SelectMenus(self)
    }
}

pub enum ActionsElement {
    Button(Button),
    Checkboxes(Checkboxes),
    DatePicker(DatePicker),
    OverflowMenu(OverflowMenu),
    PlainTextInput(PlainTextInput),
    RadioButtonGroup(RadioButtonGroup),
    SelectMenus(SelectMenus),
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

impl Into<InputElement> for SelectMenus {
    fn into(self) -> InputElement {
        InputElement::SelectMenus(self)
    }
}

pub enum InputElement {
    Checkboxes(Checkboxes),
    DatePicker(DatePicker),
    MultiSelectMenu(MultiSelectMenu),
    PlainTextInput(PlainTextInput),
    RadioButtonGroup(RadioButtonGroup),
    SelectMenus(SelectMenus),
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
