use std::default::default;

use paste::paste;

use crate::{SpacedList, Skeleton, Spacing, Position, Iter, RangeSpacedList};

spaced_list!(Hollow Range);

#[allow(unused)]
impl<S: Spacing> HollowRangeSpacedList<S> {
    delegates! {
        as SpacedList<S>:

        iter(&self) -> Iter<S, Self>;

        as RangeSpacedList<S>:

        append_range(&mut self, position: S, span: S) -> Position<S, Self>;
        insert_range(&mut self, position: S, span: S) -> Position<S, Self>;
    }
}
