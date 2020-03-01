use crate::surfaces::*;

use derive_builder::Builder;

#[builder(setter(into), pattern = "owned")]
#[derive(Builder)]
pub struct View<T: ViewType> {
    pub payload: T,
    pub private_metadata: Option<String>,
}

pub trait ViewType {}

impl ViewType for HomeTab {}

impl ViewType for Modal {}
