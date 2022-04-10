use std::default::default;

use paste::paste;

use crate::{SpacedList, Skeleton, Spacing, Position};

spaced_list!(Hollow Range);

#[allow(unused)]
impl<S: Spacing> HollowRangeSpacedList<S> {
    pub fn append_range(&mut self, distance: S, span: S) -> Position<S, Self> {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S) -> Position<S, Self> {
        todo!()
    }

    delegates! {
        inflate_after(&mut self, position: S, amount: S);
        inflate_before(&mut self, position: S, amount: S);
        deflate_after(&mut self, position: S, amount: S);
        deflate_before(&mut self, position: S, amount: S);
    }

    pub fn range_starting_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_at_or_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_at(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_at_or_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_at_or_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_at(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_at_or_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }
}
