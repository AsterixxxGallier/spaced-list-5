use std::default::default;

use num_traits::zero;

use crate::{SpacedList, SpacedListSkeleton, Spacing, Todo};
use crate::spaced_lists::spaced_list::SublistData;

#[derive(Clone, Eq, PartialEq)]
pub struct FilledRangeSpacedList<S: Spacing, T> {
    skeleton: SpacedListSkeleton<S, Self>,
    elements: Vec<T>,
    size: usize,
    deep_size: usize,
    deep_length: S,
    sublist_data: Option<SublistData<S, Self>>
}

impl<S: Spacing, T> Default for FilledRangeSpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: default(),
            elements: vec![],
            size: 0,
            deep_size: 0,
            deep_length: zero(),
            sublist_data: None
        }
    }
}

impl<S: Spacing, T> SpacedList<S> for FilledRangeSpacedList<S, T> {
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

impl<S: Spacing, T> FilledRangeSpacedList<S, T> {
    pub fn new() -> Self {
        default()
    }

    pub fn element(&self, range_index: Todo) -> &T {
        todo!()
    }

    pub fn element_mut(&mut self, range_index: Todo) -> &mut T {
        todo!()
    }

    pub fn append_range(&mut self, distance: S, span: S, element: T) -> Todo {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S, element: T) -> Todo {
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

    pub fn range_starting_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_at_or_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_at(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_at_or_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_at_or_before(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_at(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_at_or_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_ending_after(&self, position: S) -> &T {
        todo!()
    }

    pub fn range_starting_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_at_or_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_at_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_at_or_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_starting_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_at_or_before_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_at_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_at_or_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }

    pub fn range_ending_after_mut(&mut self, position: S) -> &mut T {
        todo!()
    }
}
