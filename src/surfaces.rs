use crate::blocks::*;
use crate::objects::Text;

use derive_builder::Builder;

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct HomeTab {
    pub blocks: Vec<HomeTabBlock>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Modal {
    pub title: Text,
    pub close: Option<Text>,
    pub submit: Option<Text>,
    pub blocks: Vec<ModalBlock>,
}

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct Message {}

pub enum ModalBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    Image(Image),
    Input(Input),
    Section(Section),
}

pub enum HomeTabBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    Image(Image),
    Section(Section),
}

pub enum MessageBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    File(File),
    Image(Image),
    Section(Section),
}
