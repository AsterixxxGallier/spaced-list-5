use std::rc::Rc;
use std::cell::RefCell;
use itertools::Itertools;
use crate::{Skeleton, Node, Position, HollowPosition, PushError, SpacingError, Spacing,
            BackwardsIter, ForwardsIter};
use crate::spaced_lists::{spacing_methods, trivial_accessors};

pub struct HollowSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<Node, S, ()>>>,
    size: usize,
}

impl<S: Spacing> Default for HollowSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing> HollowSpacedList<S> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_push(&mut self, spacing: S) -> Result<HollowPosition<Node, S>, PushError> {
        self.size += 1;
        let position: Position<Node, S, ()> =
            Skeleton::<Node, _, _>::try_push(self.skeleton.clone(), spacing, ())?.into();
        Ok(position.into())
    }

    // cannot fail
    pub fn insert(&mut self, position: S) -> HollowPosition<Node, S> {
        self.size += 1;
        let position: Position<Node, S, ()> =
            Skeleton::<Node, _, _>::insert(self.skeleton.clone(), position, ()).into();
        position.into()
    }


    spacing_methods!();


    #[must_use]
    pub fn first(&self) -> Option<HollowPosition<Node, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_start(self.skeleton.clone()))
        }
    }

    #[must_use]
    pub fn last(&self) -> Option<HollowPosition<Node, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_end(self.skeleton.clone()))
        }
    }


    pub fn before(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn at_or_before(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn at(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn at_or_after(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn after(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        ForwardsIter::from_start(self.skeleton.clone()).map_into()
    }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        ForwardsIter::from_start(self.skeleton).map_into()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        BackwardsIter::from_end(self.skeleton.clone()).map_into()
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        BackwardsIter::from_end(self.skeleton).map_into()
    }

    trivial_accessors!();
}
