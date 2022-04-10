use std::default::default;

use paste::paste;

use crate::{SpacedList, Skeleton, Spacing, Todo};

spaced_list!(Filled Range);

#[allow(unused)]
impl<S: Spacing, T> FilledRangeSpacedList<S, T> {
    pub fn element(&self, range_index: Todo) -> &T {
        todo!()
    }

    pub fn element_mut(&mut self, range_index: Todo) -> &mut T {
        todo!()
    }

    pub fn append_range(&mut self, distance: S, span: S, element: T) -> Todo {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S, element: T) -> Todo {
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

    pub fn range_starting_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_at_or_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_at(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_at_or_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_at_or_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_at(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_at_or_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_at_or_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_at_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_at_or_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_at_or_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_at_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_at_or_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }
}
