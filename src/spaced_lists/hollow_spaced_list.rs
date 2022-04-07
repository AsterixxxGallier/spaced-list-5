use std::default::default;

use crate::{Iter, Position, SpacedList, SpacedListSkeleton, Spacing};

#[derive(Clone, Eq, PartialEq)]
pub struct HollowSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>,
}

impl<S: Spacing> Default for HollowSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: default(),
        }
    }
}

impl<S: Spacing> SpacedList<S> for HollowSpacedList<S> {
    fn skeleton(&self) -> &SpacedListSkeleton<S, Self> {
        &self.skeleton
    }

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self> {
        &mut self.skeleton
    }
}

macro_rules! delegate {
    ($fn:ident ($($param:ident : $param_type:ty),*)$( -> $return:ty)?) => {
        pub fn $fn($($param: $param_type),*)$( -> $return)? {
            <Self as SpacedList<S>>::$fn($($param),*)
        }
    }
}

impl<S: Spacing> HollowSpacedList<S> {
    pub fn new() -> Self {
        default()
    }

    delegate!(iter (self: &mut Self) -> Iter<S, Self>);

    delegate!(append_node (self: &mut Self, distance: S) -> Position<S, Self>);
    delegate!(insert_node (self: &mut Self, position: S) -> Position<S, Self>);

    delegate!(inflate_after (self: &mut Self, position: S, amount: S));
    delegate!(inflate_before (self: &mut Self, position: S, amount: S));
    delegate!(deflate_after (self: &mut Self, position: S, amount: S));
    delegate!(deflate_before (self: &mut Self, position: S, amount: S));

    delegate!(node_before(self: &Self, position: S) -> Option<Position<S, Self>>);
    delegate!(node_at_or_before(self: &Self, position: S) -> Option<Position<S, Self>>);
    delegate!(node_at(self: &Self, position: S) -> Option<Position<S, Self>>);
    delegate!(node_at_or_after(self: &Self, position: S) -> Option<Position<S, Self>>);
    delegate!(node_after(self: &Self, position: S) -> Option<Position<S, Self>>);
}
