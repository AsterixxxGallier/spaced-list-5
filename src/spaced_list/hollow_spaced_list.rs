use paste::paste;
use num_traits::zero;

use crate::{Iter, Position, SpacedList, CrateSpacedList, Spacing};

spaced_list!(Hollow);

impl<S: Spacing> HollowSpacedList<S> {
    delegates! {
        as CrateSpacedList<S>:

        iter(&self) -> Iter<S, Self>;

        append_node(&mut self, distance: S) -> Position<S, Self>;
        insert_node(&mut self, position: S) -> Position<S, Self>;
    }
}
