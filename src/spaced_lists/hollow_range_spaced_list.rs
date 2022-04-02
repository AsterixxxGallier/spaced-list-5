use std::default::default;

use num_traits::zero;

use crate::{SpacedList, SpacedListSkeleton, Spacing, Todo};
use crate::spaced_lists::spaced_list::SublistData;

#[derive(Clone, Eq, PartialEq)]
pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>,
    size: usize,
    deep_size: usize,
    deep_length: S,
    sublist_data: Option<SublistData<S, Self>>
}

impl<S: Spacing> Default for HollowRangeSpacedList<S> {
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

impl<S: Spacing> SpacedList<S> for HollowRangeSpacedList<S> {
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

impl<S: Spacing> HollowRangeSpacedList<S> {
    pub fn new() -> Self {
        default()
    }

    pub fn append_range(&mut self, distance: S, span: S) -> Todo {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S) -> Todo {
        todo!()
    }

    pub fn inflate_after_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn inflate_before_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn inflate_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_after_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_before_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn deflate_range(&mut self, range_index: Todo, amount: S) {
        todo!()
    }

    pub fn range_starting_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_at_or_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_at(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_at_or_after(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_starting_after(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_at_or_before(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_at(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_at_or_after(&self, position: S) -> Todo {
        todo!()
    }

    pub fn range_ending_after(&self, position: S) -> Todo {
        todo!()
    }
}
