use std::default::default;

use num_traits::zero;

use crate::{Position, SpacedList, SpacedListSkeleton, Spacing, Todo};

#[derive(Clone, Eq, PartialEq)]
pub struct HollowSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>,
    size: usize,
    deep_size: usize,
    index_in_super_list: Option<usize>,
}

impl<S: Spacing> Default for HollowSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: default(),
            size: 0,
            deep_size: 0,
            index_in_super_list: None,
        }
    }
}

impl<S: Spacing> SpacedList<S> for HollowSpacedList<S> {
    fn index_in_super_list(&self) -> Option<usize> {
        self.index_in_super_list
    }

    fn set_index_in_super_list(&mut self, index: usize) {
        self.index_in_super_list = Some(index)
    }

    fn skeleton(&self) -> &SpacedListSkeleton<S, Self> {
        &self.skeleton
    }

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self> {
        &mut self.skeleton
    }

    fn size(&self) -> usize {
        self.size
    }

    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }

    fn deep_size(&self) -> usize {
        self.deep_size
    }

    fn deep_size_mut(&mut self) -> &mut usize {
        &mut self.deep_size
    }
}

macro_rules! delegate {
    ($fn:ident ($($param:ident : $param_type:ty),*)) => {
        pub fn $fn($($param: $param_type),*) {
            <Self as SpacedList<S>>::$fn($($param),*);
        }
    };
    ($fn:ident ($($param:ident : $param_type:ty),*) -> $return:ty) => {
        pub fn $fn($($param: $param_type),*) -> $return {
            <Self as SpacedList<S>>::$fn($($param),*)
        }
    };
}

impl<S: Spacing> HollowSpacedList<S> {
    pub fn new() -> Self {
        default()
    }

    delegate!(append_node (self: &mut Self, distance: S));
    delegate!(insert_node (self: &mut Self, position: S));

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
