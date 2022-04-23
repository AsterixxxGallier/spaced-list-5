use std::cell::RefCell;
use std::rc::Rc;

use crate::{Position, Skeleton, Spacing};

// TODO impl DoubleEndedIterator for Iter

pub(crate) struct Iter<Kind, S: Spacing, T> {
    position: Option<Position<Kind, S, T>>,
}

impl<Kind, S: Spacing, T> Iter<Kind, S, T> {
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

impl<Kind, S: Spacing, T> Iterator for Iter<Kind, S, T> {
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