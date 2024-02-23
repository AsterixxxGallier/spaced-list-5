use std::rc::Rc;
use std::cell::RefCell;
use itertools::Itertools;
use crate::{Skeleton, Range, Position, HollowPosition, RangePushError, RangeInsertionError, SpacingError, Spacing, BackwardsIter, ForwardsIter, display_unwrap};

pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<Range, S, ()>>>,
    size: usize,
}

impl<S: Spacing> Default for HollowRangeSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing> HollowRangeSpacedList<S> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }


    pub fn push(&mut self, spacing: S, span: S) -> HollowPosition<Range, S> {
        display_unwrap!(self.try_push(spacing, span))
    }

    pub fn insert(&mut self, start: S, end: S) -> HollowPosition<Range, S> {
        display_unwrap!(self.try_insert(start, end))
    }

    pub fn insert_with_span(&mut self, start: S, span: S) -> HollowPosition<Range, S> {
        display_unwrap!(self.try_insert_with_span(start, span))
    }

    pub fn try_push(&mut self, spacing: S, span: S) -> Result<HollowPosition<Range, S>, RangePushError> {
        self.size += 1;
        let position: Position<Range, S, ()> =
            Skeleton::<Range, _, _>::try_push(self.skeleton.clone(), spacing, span, ())?.into();
        Ok(position.into())
    }

    pub fn try_insert(&mut self, start: S, end: S) -> Result<HollowPosition<Range, S>, RangeInsertionError> {
        self.try_insert_with_span(start, end - start)
    }

    pub fn try_insert_with_span(&mut self, start: S, span: S) -> Result<HollowPosition<Range, S>, RangeInsertionError> {
        self.size += 1;
        let position: Position<Range, S, ()> =
            Skeleton::<Range, _, _>::try_insert(self.skeleton.clone(), start, span, ())?.into();
        Ok(position.into())
    }


    spacing_functions!();


    #[must_use]
    pub fn first(&self) -> Option<HollowPosition<Range, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_start(self.skeleton.clone()))
        }
    }

    #[must_use]
    pub fn last(&self) -> Option<HollowPosition<Range, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }


    pub fn starting_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }


    pub fn ending_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        ForwardsIter::from_start(self.skeleton.clone()).map_into()
    }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        ForwardsIter::from_start(self.skeleton).map_into()
    }

    pub fn iter_ranges(&self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.into_iter().tuples()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        BackwardsIter::from_end(self.skeleton.clone()).map_into()
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        BackwardsIter::from_end(self.skeleton).map_into()
    }

    pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.into_iter_backwards().tuples()
    }


    trivial_accessors!();
}
