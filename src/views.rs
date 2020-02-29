use crate::surfaces::*;

pub struct View<T: ViewType> {
    payload: T,
    private_metadata: Option<String>,
}

pub trait ViewType {}

impl ViewType for HomeTab {}

impl ViewType for Modal {}
