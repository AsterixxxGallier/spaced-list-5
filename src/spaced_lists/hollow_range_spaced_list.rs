use std::ops::Neg;
use crate::{SpacedList, Todo};
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>
}

impl<S: Spacing> HollowRangeSpacedList<S> {
    fn append_range(&mut self, distance: S, span: S) -> Todo {
        todo!()
    }

    fn insert_range(&mut self, position: S, span: S) -> Todo {
        todo!()
    }

    fn inflate_after_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    fn inflate_before_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    fn inflate_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    fn deflate_after_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    fn deflate_before_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    fn deflate_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    fn range_starting_before(&self, position: S) -> Todo {
        todo!()
    }

    fn range_starting_at_or_before(&self, position: S) -> Todo {
        todo!()
    }

    fn range_starting_at(&self, position: S) -> Todo {
        todo!()
    }

    fn range_starting_at_or_after(&self, position: S) -> Todo {
        todo!()
    }

    fn range_starting_after(&self, position: S) -> Todo {
        todo!()
    }

    fn range_ending_before(&self, position: S) -> Todo {
        todo!()
    }

    fn range_ending_at_or_before(&self, position: S) -> Todo {
        todo!()
    }

    fn range_ending_at(&self, position: S) -> Todo {
        todo!()
    }

    fn range_ending_at_or_after(&self, position: S) -> Todo {
        todo!()
    }

    fn range_ending_after(&self, position: S) -> Todo {
        todo!()
    }
}

impl<S: Spacing> SpacedList<S> for HollowRangeSpacedList<S> {

}