use std::default::default;

use num_traits::zero;

use crate::{Position, SpacedList, SpacedListSkeleton, Spacing, Todo};
use crate::spaced_lists::spaced_list::SublistData;

#[derive(Clone, Eq, PartialEq)]
pub struct HollowSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>,
    size: usize,
    deep_size: usize,
    deep_length: S,
    sublist_data: Option<SublistData<S>>
}

impl<S: Spacing> Default for HollowSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: default(),
            size: 0,
            deep_size: 0,
            deep_length: zero(),
            sublist_data: None
        }
    }
}

impl<S: Spacing> SpacedList<S> for HollowSpacedList<S> {
    fn sublist_data(&self) -> Option<&SublistData<S>> {
        self.sublist_data.as_ref()
    }

    fn add_sublist_data(&mut self, data: SublistData<S>) {
        self.sublist_data = Some(data)
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

    fn deep_length(&self) -> S {
        self.deep_length
    }

    fn deep_length_mut(&mut self) -> &mut S {
        &mut self.deep_length
    }
}

impl<S: Spacing> HollowSpacedList<S> {
    pub fn new() -> Self {
        default()
    }

    pub fn append_node(&mut self, distance: S) {
        <Self as SpacedList<S>>::append_node(self, distance);
    }

    pub fn insert_node(&mut self, position: S) {
        <Self as SpacedList<S>>::insert_node(self, position);
    }

    pub fn inflate_after(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    pub fn inflate_before(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_after(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_before(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    pub fn node_before(&self, position: S) -> Option<Position<S, Self>> {
        <Self as SpacedList<S>>::node_before(self, position)
    }

    pub fn node_at_or_before(&self, position: S) -> Option<Position<S, Self>> {
        <Self as SpacedList<S>>::node_at_or_before(self, position)
    }

    pub fn node_at(&self, position: S) -> Option<Position<S, Self>> {
        <Self as SpacedList<S>>::node_at(self, position)
    }

    pub fn node_at_or_after(&self, position: S) -> Option<Position<S, Self>> {
        <Self as SpacedList<S>>::node_at_or_after(self, position)
    }

    pub fn node_after(&self, position: S) -> Option<Position<S, Self>> {
        <Self as SpacedList<S>>::node_after(self, position)
    }
}
