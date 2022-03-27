use std::default::default;

use crate::{SpacedList, SpacedListSkeleton, Spacing, Todo};

pub struct FilledSpacedList<S: Spacing, T> {
    skeleton: SpacedListSkeleton<S, Self>,
    elements: Vec<T>,
    size: usize,
    deep_size: usize,
}

impl<S: Spacing, T> Default for FilledSpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: default(),
            elements: vec![],
            size: 0,
            deep_size: 0,
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

    fn size(&self) -> usize {
        self.size
    }

    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }

    fn deep_size(&self) -> usize {
        self.deep_size
    }

    fn deep_size_mut(&mut self) -> &mut usize {
        &mut self.deep_size
    }
}

impl<S: Spacing, T> FilledSpacedList<S, T> {
    pub fn new() -> Self {
        default()
    }

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
        self.element(self.node_before(position))
    }

    pub fn element_at_or_before(&self, position: S) -> &T {
        self.element(self.node_at_or_before(position))
    }

    pub fn element_at(&self, position: S) -> &T {
        self.element(self.node_at(position))
    }

    pub fn element_at_or_after(&self, position: S) -> &T {
        self.element(self.node_at_or_after(position))
    }

    pub fn element_after(&self, position: S) -> &T {
        self.element(self.node_after(position))
    }

    pub fn element_before_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_before(position))
    }

    pub fn element_at_or_before_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_at_or_before(position))
    }

    pub fn element_at_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_at(position))
    }

    pub fn element_at_or_after_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_at_or_after(position))
    }

    pub fn element_after_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_after(position))
    }
}
