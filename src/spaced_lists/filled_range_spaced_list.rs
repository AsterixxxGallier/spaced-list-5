use std::default::default;

use paste::paste;

use crate::{SpacedList, Skeleton, Spacing, Position};

spaced_list!(Filled Range);

#[allow(unused)]
impl<S: Spacing, T> FilledRangeSpacedList<S, T> {
    pub fn append_range(&mut self, distance: S, span: S, element: T) -> Position<S, Self> {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S, element: T) -> Position<S, Self> {
        todo!()
    }

    delegates! {
        inflate_after(&mut self, position: S, amount: S);
        inflate_before(&mut self, position: S, amount: S);
        deflate_after(&mut self, position: S, amount: S);
        deflate_before(&mut self, position: S, amount: S);
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
