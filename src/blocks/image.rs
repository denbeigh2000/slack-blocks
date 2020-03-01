use crate::objects::Text;

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use url::Url;

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


