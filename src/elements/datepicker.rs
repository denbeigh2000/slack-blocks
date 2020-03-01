use crate::objects::{ConfirmationDialog, Text};

use derive_builder::Builder;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use chrono::{Date, Utc};

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct DatePicker {
    pub action_id: String,
    pub placeholder: Option<Text>,
    // TODO: Should we allow timezones here?
    pub initial_date: Option<Date<Utc>>,
    pub confirm: Option<ConfirmationDialog>,
}

impl Serialize for DatePicker {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut size = 1;
        if self.placeholder.is_some() {
            size += 1;
        }

        if self.initial_date.is_some() {
            size += 1;
        }

        if self.confirm.is_some() {
            size += 1;
        }

        let mut map = serializer.serialize_map(Some(size))?;
        map.serialize_entry("action_id", &self.action_id)?;
        if let Some(p) = &self.placeholder {
            map.serialize_entry("placeholder", &p)?;
        }
        if let Some(d) = &self.initial_date {
            map.serialize_entry("initial_date", &d.format("%Y-%M-%d").to_string())?;
        }
        if let Some(c) = &self.confirm {
            map.serialize_entry("confirm", &c)?;
        }
        map.end()
    }
}
