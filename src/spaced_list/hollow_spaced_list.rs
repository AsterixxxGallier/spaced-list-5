use paste::paste;
use num_traits::zero;

use crate::{Iter, Position, CrateSpacedList, Spacing};

spaced_list!(Hollow);

impl<S: Spacing> HollowSpacedList<S> {
    delegates! {
        as CrateSpacedList<S>:

        iter(&mut self) -> Iter<S, Self> where Self: SpacedList<S>;

        append_node(&mut self, distance: S) -> Position<S, Self> where Self: SpacedList<S>;
        insert_node(&mut self, position: S) -> Position<S, Self> where Self: SpacedList<S>;
    }
}
