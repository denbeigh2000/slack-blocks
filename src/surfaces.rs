use crate::blocks::*;
use crate::objects::Text;

pub struct HomeTab {
    blocks: Vec<HomeTabBlock>,
}

pub struct Modal {
    title: Text,
    close: Option<Text>,
    submit: Option<Text>,
    blocks: Vec<ModalBlock>,
}

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
