use std::default::default;

use crate::{Iter, Position, SpacedList, SpacedListSkeleton, Spacing};

#[derive(Clone, Eq, PartialEq)]
pub struct HollowSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>,
}

spaced_list_impl!(HollowSpacedList<S>);

macro_rules! delegates {
    {$fn:ident(&self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)? $(;$($rest:tt)*)?} => {
        pub fn $fn(&self$(, $param: $param_type)*)$( -> $return)? {
            <Self as SpacedList<S>>::$fn(self$(, $param)*)
        }

        delegates! {
            $($($rest)*)?
        }
    };
    {$fn:ident(&mut self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)? $(;$($rest:tt)*)?} => {
        pub fn $fn(&mut self$(, $param: $param_type)*)$( -> $return)? {
            <Self as SpacedList<S>>::$fn(self$(, $param)*)
        }

        delegates! {
            $($($rest)*)?
        }
    };
    {} => {}
}

impl<S: Spacing> HollowSpacedList<S> {
    default_as_new!();

    delegates! {
        iter(&mut self) -> Iter<S, Self>;

        append_node(&mut self, distance: S) -> Position<S, Self>;
        insert_node(&mut self, position: S) -> Position<S, Self>;

        inflate_after(&mut self, position: S, amount: S);
        inflate_before(&mut self, position: S, amount: S);
        deflate_after(&mut self, position: S, amount: S);
        deflate_before(&mut self, position: S, amount: S);

        node_before(&self, position: S) -> Option<Position<S, Self>>;
        node_at_or_before(&self, position: S) -> Option<Position<S, Self>>;
        node_at(&self, position: S) -> Option<Position<S, Self>>;
        node_at_or_after(&self, position: S) -> Option<Position<S, Self>>;
        node_after(&self, position: S) -> Option<Position<S, Self>>;
    }
}
