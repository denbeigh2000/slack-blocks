use crate::surfaces::*;

pub struct View<T: ViewType> {
    pub payload: T,
    pub private_metadata: Option<String>,
}

impl<T: ViewType> View<T> {
    pub fn new(payload: T) -> View<T> {
        View {
            payload,
            private_metadata: None,
        }
    }

    pub fn new_with_metadata(payload: T, private_metadata: String) -> View<T> {
        View {
            payload,
            private_metadata: Some(private_metadata),
        }
    }
}

pub trait ViewType {}

impl ViewType for HomeTab {}

impl ViewType for Modal {}
