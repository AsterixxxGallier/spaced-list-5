use std::cell::{Ref, RefCell};
use std::default::default;
use std::rc::Rc;

use num_traits::zero;

use crate::{Position, SpacedList, SpacedListSkeleton, Spacing, Todo};
use crate::spaced_lists::spaced_list::SublistData;

#[derive(Clone)]
pub struct HollowSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>,
    size: usize,
    deep_size: usize,
    deep_length: S,
    sublist_data: Option<SublistData<S, Self>>
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
    fn sublist_data(&self) -> Option<&SublistData<S, Self>> {
        self.sublist_data.as_ref()
    }

    fn add_sublist_data(&mut self, data: SublistData<S, Self>) {
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

    pub fn insert_node(this: Rc<RefCell<Self>>, position: S) {
        <Self as SpacedList<S>>::insert_node(this, position);
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

    #[allow(clippy::needless_lifetimes)]
    pub fn node_before<'a>(self: Ref<'a, Self>, position: S) -> Option<Position<'a, S, Self>> {
        <Self as SpacedList<S>>::node_before(self, position)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn node_at_or_before<'a>(self: Ref<'a, Self>, position: S) -> Option<Position<'a, S, Self>> {
        <Self as SpacedList<S>>::node_at_or_before(self, position)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn node_at<'a>(self: Ref<'a, Self>, position: S) -> Option<Position<'a, S, Self>> {
        <Self as SpacedList<S>>::node_at(self, position)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn node_at_or_after<'a>(self: Ref<'a, Self>, position: S) -> Option<Position<'a, S, Self>> {
        <Self as SpacedList<S>>::node_at_or_after(self, position)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn node_after<'a>(self: Ref<'a, Self>, position: S) -> Option<Position<'a, S, Self>> {
        <Self as SpacedList<S>>::node_after(self, position)
    }
}
