use std::default::default;
use std::ops::Neg;
use crate::{SpacedList, Todo};
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>
}

impl<S: Spacing> HollowRangeSpacedList<S> {
    pub fn new() -> Self {
        default()
    }

    pub fn append_range(&mut self, distance: S, span: S) -> Todo {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S) -> Todo {
        todo!()
    }

    pub fn inflate_after_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn inflate_before_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn inflate_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_after_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_before_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn range_starting_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_at_or_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_at(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_at_or_after(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_after(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_at_or_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_at(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_at_or_after(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_after(&self, position: S) -> Todo {
        todo!()
    }
}

impl<S: Spacing> Default for HollowRangeSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: default()
        }
    }
}

impl<S: Spacing> SpacedList<S> for HollowRangeSpacedList<S> {
    fn skeleton(&self) -> &SpacedListSkeleton<S, Self> {
        &self.skeleton
    }

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self> {
        &mut self.skeleton
    }
}