use crate::objects::Text;
use crate::elements::*;

use url::Url;

pub struct Actions {
    elements: Vec<ActionsElement>,
    block_id: Option<String>,
}

pub struct Context {
    elements: Vec<ContextElement>,
    block_id: Option<String>,
}

pub struct Divider {
    block_id: Option<String>,
}

pub struct File {
    external_id: String,
    block_id: Option<String>,
}

pub struct Image {
    image_url: Url,
    alt_text: String,
    title: Option<Text>,
    block_id: Option<String>,
}

pub struct Input {
    label: Text,
    element: InputElement,
    block_id: Option<String>,
    // This must be plain-text, is there any way to enforce this at
    // compile-time?
    hint: Option<Text>,
    optional: Option<bool>,
}

pub struct Section {
    text: Text,
    block_id: Option<String>,
    fields: Option<Vec<Text>>,
    accessory: Option<SectionElement>,
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

pub enum ActionsElement {
    Button(Button),
    Checkboxes(Checkboxes),
    DatePicker(DatePicker),
    OverflowMenu(OverflowMenu),
    PlainTextInput(PlainTextInput),
    RadioButtonGroup(RadioButtonGroup),
    SelectMenus(SelectMenus),
}

pub enum InputElement {
    Checkboxes(Checkboxes),
    DatePicker(DatePicker),
    MultiSelectMenu(MultiSelectMenu),
    PlainTextInput(PlainTextInput),
    RadioButtonGroup(RadioButtonGroup),
    SelectMenus(SelectMenus),
}

pub enum ContextElement {
    Image(Image),
    Text(Text),
}
