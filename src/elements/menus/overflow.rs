use crate::objects::{ConfirmationDialog, OptionInput};
use serde::Serialize;

#[derive(Serialize)]
pub struct OverflowMenu {
    action_id: String,
    options: Vec<OptionInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl OverflowMenu {
    pub fn builder<S: Into<String>>(action_id: S, options: Vec<OptionInput>) -> OverflowMenuBuilder {
        OverflowMenuBuilder::new(action_id, options)
    }
}

pub struct OverflowMenuBuilder  {
    action_id: String,
    options: Vec<OptionInput>,
    confirm: Option<ConfirmationDialog>,
}

impl OverflowMenuBuilder {
    pub fn new<S: Into<String>>(action_id: S, options: Vec<OptionInput>) -> Self {
        Self {
            action_id: action_id.into(),
            options,
            confirm: None,
        }
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> OverflowMenu {
        OverflowMenu {
            action_id: self.action_id,
            options: self.options,
            confirm: self.confirm,
        }
    }
}
