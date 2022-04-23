use std::cell::RefCell;
use std::rc::Rc;

use crate::{ParentData, Position, Skeleton, Spacing};

// TODO impl DoubleEndedIterator for Iter

pub(crate) struct Iter<Kind, S: Spacing, T> {
    position: Position<Kind, S, T>,
    finished: bool,
}

impl<Kind, S: Spacing, T> Iter<Kind, S, T> {
    pub(crate) fn from(position: Position<Kind, S, T>) -> Self {
        Self {
            position,
            finished: false,
        }
    }

    pub(crate) fn from_start(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        Self {
            position: Position::at_start(skeleton),
            finished: false,
        }
    }
}

impl<Kind, S: Spacing, T> Iterator for Iter<Kind, S, T> {
    type Item = Position<Kind, S, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None
        }

        let last_position = self.position.clone();

        if self.position.index == self.position.skeleton.borrow().links.len() {
            if let Some(ParentData { parent, index_in_parent }) =
                &self.position.skeleton.clone().borrow().parent_data {
                self.position.position -= self.position.skeleton.borrow().last_position();
                self.position.skeleton = parent.upgrade().unwrap();
                self.position.position += self.position.skeleton.borrow().link(*index_in_parent);
                self.position.index = index_in_parent + 1;
            } else {
                self.finished = true;
            }
        } else if let Some(sub) =
            self.position.skeleton.clone().borrow().sub(self.position.index) {
            self.position.skeleton = sub;
            self.position.index = 0;
            self.position.position += self.position.skeleton.borrow().offset;
        } else {
            self.position.position += self.position.skeleton.borrow().link(self.position.index);
            self.position.index += 1;
        }

        Some(last_position)
    }
}