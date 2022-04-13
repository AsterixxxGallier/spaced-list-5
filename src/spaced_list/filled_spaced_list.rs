use std::default::default;

use paste::paste;

use crate::{SpacedList, Skeleton, Spacing, Position};

spaced_list!(Filled);

#[allow(unused)]
impl<S: Spacing, T> FilledSpacedList<S, T> {
    pub fn append_element(&mut self, distance: S, element: T) {
        todo!()
    }

    pub fn insert_element(&mut self, position: S, element: T) {
        todo!()
    }

    pub fn element_before(&self, position: S) -> Option<&T> {
        Some(self.element(self.node_before(position)?))
    }

    pub fn element_at_or_before(&self, position: S) -> Option<&T> {
        Some(self.element(self.node_at_or_before(position)?))
    }

    pub fn element_at(&self, position: S) -> Option<&T> {
        Some(self.element(self.node_at(position)?))
    }

    pub fn element_at_or_after(&self, position: S) -> Option<&T> {
        Some(self.element(self.node_at_or_after(position)?))
    }

    pub fn element_after(&self, position: S) -> Option<&T> {
        Some(self.element(self.node_after(position)?))
    }

    pub fn element_before_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_at_or_before_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_at_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_at_or_after_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_after_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }
}
