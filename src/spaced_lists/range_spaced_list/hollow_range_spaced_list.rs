use std::default::default;

use paste::paste;

use crate::{SpacedList, Skeleton, Spacing, Position, Iter, RangeSpacedList};
use crate::spaced_lists::positions::shallow::ShallowPosition;
use crate::spaced_lists::traversal::*;

spaced_list!(Hollow Range);

// TODO add a RangeSpacedList trait and move the method implementations below there
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
