use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use maybe_owned::MaybeOwned;

use crate::{ParentData, BoundType, EphemeralIndex, Position, Node, RangeKind, Skeleton, Spacing};

pub(crate) struct EphemeralPosition<Kind, S: Spacing, T> {
    pub(crate) skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,

    pub(crate) index: usize,
    pub(crate) position: S,
}

impl<Kind, S: Spacing, T> Clone for EphemeralPosition<Kind, S, T> {
    fn clone(&self) -> Self {
        Self {
            skeleton: self.skeleton.clone(),
            index: self.index,
            position: self.position,
        }
    }
}

impl<Kind, S: Spacing, T> EphemeralPosition<Kind, S, T> {
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

    pub(crate) fn into_index(self) -> EphemeralIndex<Kind, S, T> {
        EphemeralIndex::new(self.skeleton, self.index)
    }

    pub(crate) fn index(&self) -> EphemeralIndex<Kind, S, T> {
        EphemeralIndex::new(self.skeleton.clone(), self.index)
    }

    pub(crate) fn persistent(&self) -> Position<Kind, S, T> {
        self.skeleton.borrow().into_persistent.get(&self.index).cloned()
            .map_or(
                Position::new(self.skeleton.clone(), self.index as isize, self.position),
                |index| Position::new(index.skeleton, index.index, self.position),
            )
    }

    pub(crate) fn into_next(self) -> Option<Self> {
        if self.index == self.skeleton.borrow().links.len() {
            if let Some(ParentData { parent, index_in_parent }) =
                &self.skeleton.clone().borrow().parent_data {
                let parent = parent.upgrade().unwrap();
                let position = self.position
                    - self.skeleton.borrow().last_position()
                    + parent.borrow().link(*index_in_parent);
                Some(Self {
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
            Some(Self {
                skeleton: sub,
                index: 0,
                position,
            })
        } else {
            let position = self.position + self.skeleton.borrow().link(self.index);
            Some(Self {
                skeleton: self.skeleton,
                index: self.index + 1,
                position,
            })
        }
    }

    pub(crate) fn into_previous(self) -> Option<Self> {
        if self.index == 0 {
            if let Some(ParentData { parent, index_in_parent }) =
                &self.skeleton.clone().borrow().parent_data {
                let parent = parent.upgrade().unwrap();
                let position = self.position - self.skeleton.borrow().offset();
                Some(Self {
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
            Some(Self {
                skeleton: sub,
                index,
                position,
            })
        } else {
            let position = self.position - self.skeleton.borrow().link(self.index - 1);
            Some(Self {
                skeleton: self.skeleton,
                index: self.index - 1,
                position,
            })
        }
    }
}

impl<S: Spacing, T> EphemeralPosition<Node, S, T> {
    pub(crate) fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index])
    }

    pub(crate) fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index])
    }
}

impl<Kind: RangeKind, S: Spacing, T> EphemeralPosition<Kind, S, T> {
    pub(crate) fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index / 2])
    }

    pub(crate) fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index / 2])
    }

    pub(crate) fn bound_type(&self) -> BoundType {
        BoundType::of(self.index.try_into().unwrap())
    }

    pub(crate) fn span(&self) -> S {
        self.skeleton.borrow().links[self.index & !1]
    }

    pub(crate) fn into_range(self) -> (Self, Self) {
        match self.bound_type() {
            BoundType::Start => {
                let end = Self::new(
                    self.skeleton.clone(),
                    self.index + 1,
                    self.position + self.span());
                (self, end)
            }
            BoundType::End => {
                let start = Self::new(
                    self.skeleton.clone(),
                    self.index - 1,
                    self.position - self.span());
                (start, self)
            }
        }
    }

    pub(crate) fn range(&self) -> (MaybeOwned<Self>, MaybeOwned<Self>) {
        match self.bound_type() {
            BoundType::Start => {
                let end = Self::new(
                    self.skeleton.clone(),
                    self.index + 1,
                    self.position + self.span());
                (self.into(), end.into())
            }
            BoundType::End => {
                let start = Self::new(
                    self.skeleton.clone(),
                    self.index - 1,
                    self.position - self.span());
                (start.into(), self.into())
            }
        }
    }
}

impl<Kind, S: Spacing, T> From<Position<Kind, S, T>> for EphemeralPosition<Kind, S, T> {
    fn from(persistent: Position<Kind, S, T>) -> Self {
        persistent.ephemeral()
    }
}
