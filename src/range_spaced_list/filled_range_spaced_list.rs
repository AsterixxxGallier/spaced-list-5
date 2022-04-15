use paste::paste;
use num_traits::zero;

use crate::{SpacedList, Spacing, Position, RangeSpacedList};

spaced_list!(Filled Range);

#[allow(unused)]
impl<S: Spacing, T> FilledRangeSpacedList<S, T> {
    pub fn append_range(&mut self, distance: S, span: S, element: T) -> Position<S, Self> {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S, element: T) -> Position<S, Self> {
        todo!()
    }

    pub fn element_of_range_starting_before(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_starting_before(position)?))
    }

    pub fn element_of_range_starting_at_or_before(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_starting_at_or_before(position)?))
    }

    pub fn element_of_range_starting_at(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_starting_at(position)?))
    }

    pub fn element_of_range_starting_at_or_after(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_starting_at_or_after(position)?))
    }

    pub fn element_of_range_starting_after(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_starting_after(position)?))
    }

    pub fn element_of_range_ending_before(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_ending_before(position)?))
    }

    pub fn element_of_range_ending_at_or_before(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_ending_at_or_before(position)?))
    }

    pub fn element_of_range_ending_at(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_ending_at(position)?))
    }

    pub fn element_of_range_ending_at_or_after(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_ending_at_or_after(position)?))
    }

    pub fn element_of_range_ending_after(&self, position: S) -> Option<&T> {
        Some(self.element(self.range_ending_after(position)?))
    }

    pub fn element_of_range_starting_before_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_starting_at_or_before_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_starting_at_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_starting_at_or_after_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_starting_after_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_ending_before_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_ending_at_or_before_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_ending_at_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_ending_at_or_after_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }

    pub fn element_of_range_ending_after_mut(&mut self, position: S) -> Option<&mut T> {
        todo!()
    }
}
