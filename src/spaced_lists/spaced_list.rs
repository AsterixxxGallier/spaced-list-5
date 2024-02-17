use std::rc::Rc;
use std::cell::RefCell;
use crate::{Skeleton, Node, Position, PushError, SpacingError, Spacing, BackwardsIter,
            ForwardsIter};
use crate::spaced_lists::{spacing_methods, trivial_accessors};

pub struct SpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Node, S, T>>>,
    size: usize,
}

impl<S: Spacing, T> Default for SpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing, T> SpacedList<S, T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_push(&mut self, spacing: S, value: T) -> Result<Position<Node, S, T>, PushError> {
        self.size += 1;
        Ok(Skeleton::<Node, _, _>::try_push(self.skeleton.clone(), spacing, value)?.into())
    }

    // cannot fail
    pub fn insert(&mut self, position: S, value: T) -> Position<Node, S, T> {
        self.size += 1;
        Skeleton::<Node, _, _>::insert(self.skeleton.clone(), position, value).into()
    }


    spacing_methods!();


    #[must_use]
    pub fn first(&self) -> Option<Position<Node, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_start(self.skeleton.clone()))
        }
    }

    #[must_use]
    pub fn last(&self) -> Option<Position<Node, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_end(self.skeleton.clone()))
        }
    }


    pub fn before(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn at_or_before(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::at_or_before(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn at(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::at(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn at_or_after(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::at_or_after(self.skeleton.clone(), position).map(Into::into)
    }

    pub fn after(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::after(self.skeleton.clone(), position).map(Into::into)
    }


    pub fn iter(&self) -> impl Iterator<Item=Position<Node, S, T>> {
        ForwardsIter::from_start(self.skeleton.clone())
    }

    pub fn into_iter(self) -> impl Iterator<Item=Position<Node, S, T>> {
        ForwardsIter::from_start(self.skeleton)
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=Position<Node, S, T>> {
        BackwardsIter::from_end(self.skeleton.clone())
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=Position<Node, S, T>> {
        BackwardsIter::from_end(self.skeleton)
    }


    trivial_accessors!();
}
