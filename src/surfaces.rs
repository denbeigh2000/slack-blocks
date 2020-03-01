use crate::blocks::Image;
use crate::blocks::*;
use crate::objects::Text;

use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct HomeTab {
    pub blocks: Vec<HomeTabBlock>,
}

impl HomeTab {
    pub fn new(blocks: Vec<HomeTabBlock>) -> Self {
        Self { blocks }
    }
}

#[derive(Serialize)]
pub struct Modal {
    pub title: Text,
    pub close: Option<Text>,
    pub submit: Option<Text>,
    pub blocks: Vec<ModalBlock>,
}

pub struct ModalBuilder {
    title: Text,
    blocks: Vec<ModalBlock>,
    close: Option<Text>,
    submit: Option<Text>,
}

impl ModalBuilder {
    pub fn new(title: Text, blocks: Vec<ModalBlock>) -> Self {
        Self {
            title,
            blocks,
            close: None,
            submit: None,
        }
    }

    pub fn set_close(mut self, close: Text) -> Self {
        self.close = Some(close);
        self
    }

    pub fn set_submit(mut self, submit: Text) -> Self {
        self.submit = Some(submit);
        self
    }

    pub fn build(self) -> Modal {
        Modal {
            title: self.title,
            blocks: self.blocks,
            close: self.close,
            submit: self.submit,
        }
    }
}

#[derive(Serialize)]
pub struct Message {}

pub enum ModalBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    Image(Image),
    Input(Input),
    Section(Section),
}

impl Serialize for ModalBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ModalBlock::Actions(e) => e.serialize(serializer),
            ModalBlock::Context(e) => e.serialize(serializer),
            ModalBlock::Divider(e) => e.serialize(serializer),
            ModalBlock::Image(e) => e.serialize(serializer),
            ModalBlock::Input(e) => e.serialize(serializer),
            ModalBlock::Section(e) => e.serialize(serializer),
        }
    }
}

pub enum HomeTabBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    Image(Image),
    Section(Section),
}

impl Serialize for HomeTabBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            HomeTabBlock::Actions(e) => e.serialize(serializer),
            HomeTabBlock::Context(e) => e.serialize(serializer),
            HomeTabBlock::Divider(e) => e.serialize(serializer),
            HomeTabBlock::Image(e) => e.serialize(serializer),
            HomeTabBlock::Section(e) => e.serialize(serializer),
        }
    }
}

pub enum MessageBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    File(File),
    Image(Image),
    Section(Section),

}
impl Serialize for MessageBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageBlock::Actions(e) => e.serialize(serializer),
            MessageBlock::Context(e) => e.serialize(serializer),
            MessageBlock::Divider(e) => e.serialize(serializer),
            MessageBlock::File(e) => e.serialize(serializer),
            MessageBlock::Image(e) => e.serialize(serializer),
            MessageBlock::Section(e) => e.serialize(serializer),
        }
    }
}
