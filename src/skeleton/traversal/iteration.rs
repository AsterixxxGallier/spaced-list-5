use std::cell::RefCell;
use std::rc::Rc;

use crate::{Position, Skeleton, Spacing};

// TODO implement parallel iteration
// TODO use IntoIterator and DoubleEndedIterator traits where applicable

pub(crate) struct ForwardsIter<Kind, S: Spacing, T> {
    position: Option<Position<Kind, S, T>>,
}

impl<Kind, S: Spacing, T> ForwardsIter<Kind, S, T> {
    pub(crate) fn from(position: Position<Kind, S, T>) -> Self {
        Self {
            position: Some(position),
        }
    }

    pub(crate) fn from_start(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        Self {
            position: Some(Position::at_start(skeleton)),
        }
    }
}

impl<Kind, S: Spacing, T> Iterator for ForwardsIter<Kind, S, T> {
    type Item = Position<Kind, S, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.position.clone() {
            None => None,
            Some(pos) => {
                self.position = self.position.take().unwrap().into_next();
                Some(pos)
            }
        }
    }
}

pub(crate) struct BackwardsIter<Kind, S: Spacing, T> {
    position: Option<Position<Kind, S, T>>,
}

impl<Kind, S: Spacing, T> BackwardsIter<Kind, S, T> {
    pub(crate) fn from(position: Position<Kind, S, T>) -> Self {
        Self {
            position: Some(position),
        }
    }

    pub(crate) fn from_end(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        Self {
            position: Some(Position::at_end(skeleton)),
        }
    }
}

impl<Kind, S: Spacing, T> Iterator for BackwardsIter<Kind, S, T> {
    type Item = Position<Kind, S, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.position.clone() {
            None => None,
            Some(pos) => {
                self.position = self.position.take().unwrap().into_previous();
                Some(pos)
            }
        }
    }
}