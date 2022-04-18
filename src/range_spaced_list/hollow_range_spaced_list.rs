use paste::paste;
use num_traits::zero;

use crate::{SpacedList, CrateSpacedList, Spacing, Position, RangeSpacedList};
use crate::iteration::range::RangeIter;

spaced_list!(Hollow Range);

#[allow(unused)]
impl<S: Spacing> HollowRangeSpacedList<S> {
    delegates! {
        as RangeSpacedList<S>:

        append_range(&mut self, position: S, span: S) -> Position<S, Self>;
        insert_range(&mut self, position: S, span: S) -> Position<S, Self>;
    }

    pub fn iter(&self) -> RangeIter<S, Self> {
        RangeIter::new(self)
    }
}
