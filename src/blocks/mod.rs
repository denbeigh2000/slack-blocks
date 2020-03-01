mod actions;
mod image;
mod input;
mod section;

pub use crate::blocks::actions::*;
pub use crate::blocks::image::*;
pub use crate::blocks::input::*;
pub use crate::blocks::section::*;

use crate::objects::Text;

use serde::{Serialize, Serializer};

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

