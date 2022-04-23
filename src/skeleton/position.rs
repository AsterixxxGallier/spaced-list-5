use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use maybe_owned::MaybeOwned;

use crate::{ParentData, Spacing};
use crate::skeleton::{Range, Skeleton};

pub struct Position<Kind, S: Spacing, T> {
    pub(crate) skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,
    pub(crate) index: usize,
    pub(crate) position: S,
}

impl<Kind, S: Spacing, T> Clone for Position<Kind, S, T> {
    fn clone(&self) -> Self {
        Self {
            skeleton: self.skeleton.clone(),
            index: self.index,
            position: self.position,
        }
    }
}

impl<Kind, S: Spacing, T> Position<Kind, S, T> {
    pub(crate) fn new(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>, index: usize, position: S) -> Self {
        Self {
            skeleton,
            index,
            position,
        }
    }

    pub(crate) fn at_start(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        let position = skeleton.borrow().offset;
        Self {
            skeleton,
            index: 0,
            position,
        }
    }

    pub(crate) fn at_end(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        let index = skeleton.borrow().elements.len() - 1;
        let position = skeleton.borrow().last_position();
        Self {
            skeleton,
            index,
            position,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }

    pub fn into_next(self) -> Option<Self> {
        if self.index == self.skeleton.borrow().links.len() {
            if let Some(ParentData { parent, index_in_parent }) =
            &self.skeleton.clone().borrow().parent_data {
                let parent = parent.upgrade().unwrap();
                let position = self.position
                    - self.skeleton.borrow().last_position()
                    + parent.borrow().link(*index_in_parent);
                Some(Position {
                    skeleton: parent,
                    index: index_in_parent + 1,
                    position,
                })
            } else {
                None
            }
        } else if let Some(sub) =
        self.skeleton.clone().borrow().sub(self.index) {
            let position = self.position + sub.borrow().offset;
            Some(Position {
                skeleton: sub,
                index: 0,
                position,
            })
        } else {
            let position = self.position + self.skeleton.borrow().link(self.index);
            Some(Position {
                skeleton: self.skeleton,
                index: self.index + 1,
                position,
            })
        }
    }

    pub fn into_previous(self) -> Option<Self> {
        if self.index == 0 {
            if let Some(ParentData { parent, index_in_parent }) =
            &self.skeleton.clone().borrow().parent_data {
                let parent = parent.upgrade().unwrap();
                let position = self.position - self.skeleton.borrow().offset();
                Some(Position {
                    skeleton: parent,
                    index: *index_in_parent,
                    position,
                })
            } else {
                None
            }
        } else if let Some(sub) =
        self.skeleton.clone().borrow().sub(self.index - 1) {
            let position = self.position
                - self.skeleton.borrow().link(self.index - 1)
                + sub.borrow().last_position();
            let index = sub.borrow().links.len();
            Some(Position {
                skeleton: sub,
                index,
                position,
            })
        } else {
            let position = self.position - self.skeleton.borrow().link(self.index - 1);
            Some(Position {
                skeleton: self.skeleton,
                index: self.index - 1,
                position,
            })
        }
    }

    pub fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index])
    }

    pub fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index])
    }
}

pub struct HollowPosition<Kind, S: Spacing> {
    pub(crate) skeleton: Rc<RefCell<Skeleton<Kind, S, ()>>>,
    pub(crate) index: usize,
    pub(crate) position: S,
}

impl<Kind, S: Spacing> Clone for HollowPosition<Kind, S> {
    fn clone(&self) -> Self {
        Self {
            skeleton: self.skeleton.clone(),
            index: self.index,
            position: self.position,
        }
    }
}

impl<Kind, S: Spacing> HollowPosition<Kind, S> {
    pub(crate) fn new(skeleton: Rc<RefCell<Skeleton<Kind, S, ()>>>, index: usize, position: S) -> Self {
        Self {
            skeleton,
            index,
            position,
        }
    }

    pub(crate) fn at_start(skeleton: Rc<RefCell<Skeleton<Kind, S, ()>>>) -> Self {
        let position = skeleton.borrow().offset;
        Self {
            skeleton,
            index: 0,
            position,
        }
    }

    pub(crate) fn at_end(skeleton: Rc<RefCell<Skeleton<Kind, S, ()>>>) -> Self {
        let index = skeleton.borrow().elements.len() - 1;
        let position = skeleton.borrow().last_position();
        Self {
            skeleton,
            index,
            position,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BoundType {
    Start,
    End,
}

impl BoundType {
    pub(crate) fn of(index: usize) -> Self {
        match index & 1 {
            0 => Self::Start,
            1 => Self::End,
            _ => unreachable!()
        }
    }
}

impl<S: Spacing> HollowPosition<Range, S> {
    pub fn bound_type(&self) -> BoundType {
        BoundType::of(self.index)
    }

    pub fn span(&self) -> S {
        self.skeleton.borrow().links[self.index & !1]
    }

    pub fn into_range(self) -> (Self, Self) {
        match self.bound_type() {
            BoundType::Start => {
                let end = HollowPosition::new(
                    self.skeleton.clone(),
                    self.index + 1,
                    self.position + self.span());
                (self, end)
            }
            BoundType::End => {
                let start = HollowPosition::new(
                    self.skeleton.clone(),
                    self.index - 1,
                    self.position - self.span());
                (start, self)
            }
        }
    }

    pub fn range(&self) -> (MaybeOwned<Self>, MaybeOwned<Self>) {
        match self.bound_type() {
            BoundType::Start => {
                let end = HollowPosition::new(
                    self.skeleton.clone(),
                    self.index + 1,
                    self.position + self.span());
                (self.into(), end.into())
            }
            BoundType::End => {
                let start = HollowPosition::new(
                    self.skeleton.clone(),
                    self.index - 1,
                    self.position - self.span());
                (start.into(), self.into())
            }
        }
    }
}

impl<Kind, S: Spacing> From<Position<Kind, S, ()>> for HollowPosition<Kind, S> {
    fn from(position: Position<Kind, S, ()>) -> Self {
        Self::new(position.skeleton, position.index, position.position)
    }
}

impl<Kind, S: Spacing> From<HollowPosition<Kind, S>> for Position<Kind, S, ()> {
    fn from(position: HollowPosition<Kind, S>) -> Self {
        Self::new(position.skeleton, position.index, position.position)
    }
}