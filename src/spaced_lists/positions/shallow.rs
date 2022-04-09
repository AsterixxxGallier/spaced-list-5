use crate::{Spacing};

pub struct ShallowPosition<S: Spacing> {
    pub(crate) index: usize,
    pub(crate) position: S,
}

impl<S: Spacing> ShallowPosition<S> {
    pub(crate) fn new(index: usize, position: S) -> Self {
        Self {
            index,
            position
        }
    }
}