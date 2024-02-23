use std::rc::Rc;
use std::cell::{Ref, RefCell};
use crate::{Skeleton, NestedRange, Position, NestedRangePushError, NestedRangeInsertionError,
            SpacingError, Spacing, BackwardsIter, ForwardsIter};

pub struct NestedRangeSpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<NestedRange, S, T>>>,
    size: usize,
}

impl<S: Spacing, T> Default for NestedRangeSpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing, T> NestedRangeSpacedList<S, T> {
    #[must_use]
     pub fn new() -> Self {
        Self::default()
    }

    pub fn try_push(&mut self, spacing: S, span: S, value: T) -> Result<Position<NestedRange, S, T>, NestedRangePushError> {
        self.size += 1;
        Ok(Skeleton::<NestedRange, _, _>::try_push(self.skeleton.clone(), spacing, span, value)?.into())
    }

    pub fn try_insert(&mut self, start: S, end: S, value: T) -> Result<Position<NestedRange, S, T>, NestedRangeInsertionError> {
        self.try_insert_with_span(start, end - start, value)
    }

    pub fn try_insert_with_span(&mut self, start: S, span: S, value: T) -> Result<Position<NestedRange, S, T>, NestedRangeInsertionError> {
        self.size += 1;
        Ok(Skeleton::<NestedRange, _, _>::try_insert(self.skeleton.clone(), start, span, value)?.into())
    }


    spacing_functions!();


    #[must_use]
     pub fn first(&self) -> Option<Position<NestedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_start(self.skeleton.clone()))
        }
    }

    #[must_use]
     pub fn last(&self) -> Option<Position<NestedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::at_or_before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::at(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::at_or_after(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::after(self.skeleton.clone(), position).map(Into::into)
    }


    pub fn starting_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_at(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_at(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_after(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn starting_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_after(self.skeleton.clone(), position).map(Into::into)
    }


    pub fn ending_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn ending_at(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_at(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_after(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn ending_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_after(self.skeleton.clone(), position).map(Into::into)
    }


    pub fn conditional_starting_or_ending_before(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_before(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_or_ending_at_or_before(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_at_or_before(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_or_ending_at(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_at(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_or_ending_at_or_after(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_at_or_after(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_or_ending_after(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_after(self.skeleton.clone(), position, condition).map(Into::into)
    }


    pub fn conditional_starting_before(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_starting_before(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_at_or_before(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_starting_at_or_before(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_at(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_starting_at(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_at_or_after(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_starting_at_or_after(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_starting_after(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_starting_after(self.skeleton.clone(), position, condition).map(Into::into)
    }


    pub fn conditional_ending_before(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_ending_before(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_ending_at_or_before(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_ending_at_or_before(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_ending_at(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_ending_at(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_ending_at_or_after(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_ending_at_or_after(self.skeleton.clone(), position, condition).map(Into::into)
    }

    pub fn conditional_ending_after(&self, position: S, condition: fn(Ref<T>) -> bool) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::conditional_ending_after(self.skeleton.clone(), position, condition).map(Into::into)
    }


    pub fn iter(&self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton.clone())
    }

    pub fn into_iter(self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton)
    }

    // TODO nested iter_ranges (for the hollow version too)
    /*pub fn iter_ranges(&self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.into_iter().tuples()
    }*/

    pub fn iter_backwards(&self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton.clone())
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton)
    }

    /*pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.into_iter_backwards().tuples()
    }*/


    trivial_accessors!();
}
