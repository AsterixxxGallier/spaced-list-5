use std::default::default;

use crate::{SpacedList, SpacedListSkeleton, Spacing, Todo};

#[derive(Clone, Eq, PartialEq)]
pub struct FilledSpacedList<S: Spacing, T> {
    skeleton: SpacedListSkeleton<S, Self>,
    elements: Vec<T>,
}

impl<S: Spacing, T> Default for FilledSpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: default(),
            elements: vec![],
        }
    }
}

impl<S: Spacing, T> SpacedList<S> for FilledSpacedList<S, T> {
    fn skeleton(&self) -> &SpacedListSkeleton<S, Self> {
        &self.skeleton
    }

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self> {
        &mut self.skeleton
    }
}

#[allow(unused)]
impl<S: Spacing, T> FilledSpacedList<S, T> {
    default_as_new!();

    pub fn element(&self, index: Todo) -> &T {
        todo!()
    }

    pub fn element_mut(&mut self, index: Todo) -> &mut T {
        todo!()
    }

    pub fn append_element(&mut self, distance: S, element: T) {
        todo!()
    }

    pub fn insert_element(&mut self, position: S, element: T) {
        todo!()
    }

    pub fn element_before(&self, position: S) -> &T {
        // self.element(self.node_before(position))
        todo!()
    }

    pub fn element_at_or_before(&self, position: S) -> &T {
        // self.element(self.node_at_or_before(position))
        todo!()
    }

    pub fn element_at(&self, position: S) -> &T {
        // self.element(self.node_at(position))
        todo!()
    }

    pub fn element_at_or_after(&self, position: S) -> &T {
        // self.element(self.node_at_or_after(position))
        todo!()
    }

    pub fn element_after(&self, position: S) -> &T {
        // self.element(self.node_after(position))
        todo!()
    }

    pub fn element_before_mut(&mut self, position: S) -> &mut T {
        // self.element_mut(self.node_before(position))
        todo!()
    }

    pub fn element_at_or_before_mut(&mut self, position: S) -> &mut T {
        // self.element_mut(self.node_at_or_before(position))
        todo!()
    }

    pub fn element_at_mut(&mut self, position: S) -> &mut T {
        // self.element_mut(self.node_at(position))
        todo!()
    }

    pub fn element_at_or_after_mut(&mut self, position: S) -> &mut T {
        // self.element_mut(self.node_at_or_after(position))
        todo!()
    }

    pub fn element_after_mut(&mut self, position: S) -> &mut T {
        // self.element_mut(self.node_after(position))
        todo!()
    }
}
