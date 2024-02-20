use std::rc::Rc;
use std::cell::RefCell;
use itertools::Itertools;
use crate::{Skeleton, NestedRange, Position, HollowPosition, NestedRangePushError,
            NestedRangeInsertionError, SpacingError, Spacing, BackwardsIter, ForwardsIter};

pub struct HollowNestedRangeSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<NestedRange, S, ()>>>,
    size: usize,
}

impl<S: Spacing> Default for HollowNestedRangeSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing> HollowNestedRangeSpacedList<S> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_push(&mut self, spacing: S, span: S) -> Result<HollowPosition<NestedRange, S>, NestedRangePushError> {
        self.size += 1;
        let position: Position<NestedRange, S, ()> =
            Skeleton::<NestedRange, _, _>::try_push(self.skeleton.clone(), spacing, span, ())?.into();
        Ok(position.into())
    }

    pub fn try_insert(&mut self, start: S, end: S) -> Result<HollowPosition<NestedRange, S>, NestedRangeInsertionError> {
        self.try_insert_with_span(start, end - start)
    }

    pub fn try_insert_with_span(&mut self, start: S, span: S) -> Result<HollowPosition<NestedRange, S>, NestedRangeInsertionError> {
        self.size += 1;
        let position: Position<NestedRange, S, ()> =
            Skeleton::<NestedRange, _, _>::try_insert(self.skeleton.clone(), start, span, ())?.into();
        Ok(position.into())
    }


    spacing_methods!();


    #[must_use]
    pub fn first(&self) -> Option<HollowPosition<NestedRange, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_start(self.skeleton.clone()))
        }
    }

    #[must_use]
    pub fn last(&self) -> Option<HollowPosition<NestedRange, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }


    pub fn starting_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }


    pub fn ending_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        ForwardsIter::from_start(self.skeleton.clone()).map_into()
    }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        ForwardsIter::from_start(self.skeleton).map_into()
    }

    /*pub fn iter_ranges(&self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.into_iter().tuples()
    }*/

    pub fn iter_backwards(&self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        BackwardsIter::from_end(self.skeleton.clone()).map_into()
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        BackwardsIter::from_end(self.skeleton).map_into()
    }

    /*pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.into_iter_backwards().tuples()
    }*/


    trivial_accessors!();
}