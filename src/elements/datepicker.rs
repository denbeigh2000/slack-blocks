use crate::objects::{ConfirmationDialog, Text};

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use chrono::{Date, Utc};

pub struct DatePicker {
    pub action_id: String,
    pub placeholder: Option<Text>,
    // TODO: Should we allow timezones here?
    pub initial_date: Option<Date<Utc>>,
    pub confirm: Option<ConfirmationDialog>,
}

impl DatePicker {
    pub fn builder<S: Into<String>>(action_id: S) -> DatePickerBuilder {
        DatePickerBuilder {
            action_id: action_id.into(),
            placeholder: None,
            initial_date: None,
            confirm: None,
        }
    }
}

pub struct DatePickerBuilder {
    action_id: String,
    placeholder: Option<Text>,
    initial_date: Option<Date<Utc>>,
    confirm: Option<ConfirmationDialog>,
}

impl DatePickerBuilder {
    pub fn new<S: Into<String>>(action_id: S) -> Self {
        Self {
            action_id: action_id.into(),
            placeholder: None,
            initial_date: None,
            confirm: None,
        }
    }

    pub fn set_placeholder(mut self, ph: Text) -> Self {
        self.placeholder = Some(ph);
        self
    }

    pub fn set_initial_date(mut self, init_date: Date<Utc>) -> Self {
        self.initial_date = Some(init_date);
        self
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> DatePicker {
        DatePicker {
            action_id: self.action_id,
            placeholder: self.placeholder,
            initial_date: self.initial_date,
            confirm: self.confirm,
        }
    }
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
        map.serialize_entry("type", "datepicker")?;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let picker = DatePicker::builder("action_id").build();
        let json = serde_json::to_string(&picker).unwrap();
        assert_eq!(
            json.as_str(),
            r#"{"type":"datepicker","action_id":"action_id"}"#
        );
    }
}
