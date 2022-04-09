use std::default::default;

use paste::paste;

use crate::{SpacedList, SpacedListSkeleton, Spacing, Todo};

spaced_list!(Hollow Range);

#[allow(unused)]
impl<S: Spacing> HollowRangeSpacedList<S> {
    default_as_new!();

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
