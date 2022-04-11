use std::default::default;

use crate::{Iter, Position, SpacedList, Skeleton, Spacing};
use paste::paste;

spaced_list!(Hollow);

impl<S: Spacing> HollowSpacedList<S> {
    delegates! {
        as SpacedList<S>:

        iter(&mut self) -> Iter<S, Self>;

        append_node(&mut self, distance: S) -> Position<S, Self>;
        insert_node(&mut self, position: S) -> Position<S, Self>;
    }
}
