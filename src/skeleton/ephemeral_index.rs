use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use maybe_owned::MaybeOwned;

use crate::{ParentData, BoundType, EphemeralPosition, Index, Node, RangeKind, Skeleton, Spacing};

pub(crate) struct EphemeralIndex<Kind, S: Spacing, T> {
    pub(crate) skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,

    pub(crate) index: usize,
}

impl<Kind, S: Spacing, T> Clone for EphemeralIndex<Kind, S, T> {
    fn clone(&self) -> Self {
        Self {
            skeleton: self.skeleton.clone(),
            index: self.index,
        }
    }
}

impl<Kind, S: Spacing, T> EphemeralIndex<Kind, S, T> {
    pub(crate) fn new(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>, index: usize) -> Self {
        Self {
            skeleton,
            index,
        }
    }

    pub(crate) fn at_start(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        Self {
            skeleton,
            index: 0,
        }
    }

    pub(crate) fn at_end(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        let index = skeleton.borrow().elements.len() - 1;
        Self {
            skeleton,
            index,
        }
    }

    pub(crate) fn position(&self) -> EphemeralPosition<Kind, S, T> {
        Skeleton::at_index(self.skeleton.clone(), self.index).unwrap()
    }

    pub(crate) fn persistent(&self) -> Index<Kind, S, T> {
        self.skeleton.borrow().into_persistent.get(&self.index).cloned()
            .unwrap_or(Index::new(self.skeleton.clone(), self.index as isize))
    }

    pub(crate) fn into_next(self) -> Option<Self> {
        if self.index == self.skeleton.borrow().links.len() {
            if let Some(ParentData { parent, index_in_parent }) =
            &self.skeleton.clone().borrow().parent_data {
                let parent = parent.upgrade().unwrap();
                Some(Self {
                    skeleton: parent,
                    index: index_in_parent + 1,
                })
            } else {
                None
            }
        } else if let Some(sub) =
        self.skeleton.clone().borrow().sub(self.index) {
            Some(Self {
                skeleton: sub,
                index: 0,
            })
        } else {
            Some(Self {
                skeleton: self.skeleton,
                index: self.index + 1,
            })
        }
    }

    pub(crate) fn into_previous(self) -> Option<Self> {
        if self.index == 0 {
            if let Some(ParentData { parent, index_in_parent }) =
            &self.skeleton.clone().borrow().parent_data {
                let parent = parent.upgrade().unwrap();
                Some(Self {
                    skeleton: parent,
                    index: *index_in_parent,
                })
            } else {
                None
            }
        } else if let Some(sub) =
        self.skeleton.clone().borrow().sub(self.index - 1) {
            let index = sub.borrow().links.len();
            Some(Self {
                skeleton: sub,
                index,
            })
        } else {
            Some(Self {
                skeleton: self.skeleton,
                index: self.index - 1,
            })
        }
    }
}

impl <S: Spacing, T> EphemeralIndex<Node, S, T> {
    pub(crate) fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index])
    }

    pub(crate) fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index])
    }
}

impl<Kind: RangeKind, S: Spacing, T> EphemeralIndex<Kind, S, T> {
    pub(crate) fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index / 2])
    }

    pub(crate) fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index / 2])
    }
    pub(crate) fn bound_type(&self) -> BoundType {
        BoundType::of(self.index)
    }

    pub(crate) fn span(&self) -> S {
        self.skeleton.borrow().links[self.index & !1]
    }

    pub(crate) fn into_range(self) -> (Self, Self) {
        match self.bound_type() {
            BoundType::Start => {
                let end = Self::new(
                    self.skeleton.clone(),
                    self.index + 1);
                (self, end)
            }
            BoundType::End => {
                let start = Self::new(
                    self.skeleton.clone(),
                    self.index - 1);
                (start, self)
            }
        }
    }

    pub(crate) fn range(&self) -> (MaybeOwned<Self>, MaybeOwned<Self>) {
        match self.bound_type() {
            BoundType::Start => {
                let end = Self::new(
                    self.skeleton.clone(),
                    self.index + 1);
                (self.into(), end.into())
            }
            BoundType::End => {
                let start = Self::new(
                    self.skeleton.clone(),
                    self.index - 1);
                (start.into(), self.into())
            }
        }
    }
}

impl<Kind, S: Spacing, T> From<Index<Kind, S, T>> for EphemeralIndex<Kind, S, T> {
    fn from(persistent: Index<Kind, S, T>) -> Self {
        persistent.ephemeral()
    }
}
