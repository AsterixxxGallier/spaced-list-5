use crate::{SpacedList, Todo};
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct FilledRangeSpacedList<S: Spacing, T> {
    skeleton: SpacedListSkeleton<S, Self>,
    elements: Vec<T>
}

impl<S: Spacing, T> FilledRangeSpacedList<S, T> {
    fn append_range(&mut self, distance: S, span: S, element: T) -> Todo {
        todo!()
    }

    fn insert_range(&mut self, position: S, span: S, element: T) -> Todo {
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

    fn range_starting_before(&self, position: S) -> &T {
        todo!()
    }

    fn range_starting_at_or_before(&self, position: S) -> &T {
        todo!()
    }

    fn range_starting_at(&self, position: S) -> &T {
        todo!()
    }

    fn range_starting_at_or_after(&self, position: S) -> &T {
        todo!()
    }

    fn range_starting_after(&self, position: S) -> &T {
        todo!()
    }

    fn range_ending_before(&self, position: S) -> &T {
        todo!()
    }

    fn range_ending_at_or_before(&self, position: S) -> &T {
        todo!()
    }

    fn range_ending_at(&self, position: S) -> &T {
        todo!()
    }

    fn range_ending_at_or_after(&self, position: S) -> &T {
        todo!()
    }

    fn range_ending_after(&self, position: S) -> &T {
        todo!()
    }

    fn range_starting_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_starting_at_or_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_starting_at_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_starting_at_or_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_starting_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_ending_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_ending_at_or_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_ending_at_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_ending_at_or_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    fn range_ending_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }
}

impl<S: Spacing, T> SpacedList<S> for FilledRangeSpacedList<S, T> {

}