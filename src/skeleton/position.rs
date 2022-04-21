use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use maybe_owned::MaybeOwned;

use crate::skeleton::{Range, Skeleton};
use crate::Spacing;

pub struct Position<Kind, S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,
    index: usize,
    position: S,
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

    pub(crate) fn skeleton(&self) -> &Rc<RefCell<Skeleton<Kind, S, T>>> {
        &self.skeleton
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub fn position(&self) -> S {
        self.position
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

impl<S: Spacing, T> Position<Range, S, T> {
    pub fn bound_type(&self) -> BoundType {
        BoundType::of(self.index)
    }

    pub fn span(&self) -> S {
        self.skeleton.borrow().links[self.index & !1]
    }

    pub fn into_range(self) -> (Self, Self) {
        match self.bound_type() {
            BoundType::Start => {
                let end = Position::new(
                    self.skeleton.clone(),
                    self.index + 1,
                    self.position + self.span());
                (self, end)
            }
            BoundType::End => {
                let start = Position::new(
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
                let end = Position::new(
                    self.skeleton.clone(),
                    self.index + 1,
                    self.position + self.span());
                (self.into(), end.into())
            }
            BoundType::End => {
                let start = Position::new(
                    self.skeleton.clone(),
                    self.index - 1,
                    self.position - self.span());
                (start.into(), self.into())
            }
        }
    }
}