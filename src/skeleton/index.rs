use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use maybe_owned::MaybeOwned;

use crate::{EphemeralPosition, HollowPosition, ParentData, Position, Spacing};
use crate::skeleton::{Range, Skeleton};

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

    pub(crate) fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index])
    }

    pub(crate) fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index])
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

impl<S: Spacing, T> EphemeralIndex<Range, S, T> {
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

macro_rules! index {
    ($name:ident; <Kind, S: Spacing$(, $T:ident)?>; $type:ty; $skeleton:ty) => {
        pub struct $name<Kind, S: Spacing$(, $T)?> {
            pub(crate) skeleton: Rc<RefCell<$skeleton>>,
            pub(crate) index: isize,
        }

        impl<Kind, S: Spacing$(, $T)?> Clone for $type {
            fn clone(&self) -> Self {
                Self {
                    skeleton: self.skeleton.clone(),
                    index: self.index,
                }
            }
        }

        impl<Kind, S: Spacing$(, $T)?> $type {
            pub(crate) fn new(skeleton: Rc<RefCell<$skeleton>>, index: isize) -> Self {
                Self {
                    skeleton,
                    index,
                }
            }

            pub(crate) fn at_start(skeleton: Rc<RefCell<$skeleton>>) -> Self {
                let index = skeleton.borrow().first_persistent_index;
                Self {
                    skeleton,
                    index,
                }
            }

            pub(crate) fn at_end(skeleton: Rc<RefCell<$skeleton>>) -> Self {
                let index = (skeleton.borrow().elements.len() - 1) as isize;
                Self {
                    skeleton,
                    index,
                }
            }

            /*pub fn iter_next(&self) -> impl Iterator<Item = Self> {
                ForwardsIter::from(self.clone().into()).map_into()
            }

            pub fn into_iter_next(self) -> impl Iterator<Item = Self> {
                ForwardsIter::from(self.into()).map_into()
            }

            pub fn iter_previous(&self) -> impl Iterator<Item = Self> {
                BackwardsIter::from(self.clone().into()).map_into()
            }

            pub fn into_iter_previous(self) -> impl Iterator<Item = Self> {
                BackwardsIter::from(self.into()).map_into()
            }

            pub fn into_next(self) -> Option<Self> {
                self.ephemeral().into_next().map(|ephemeral| ephemeral.persistent().into())
            }

            pub fn into_previous(self) -> Option<Self> {
                self.ephemeral().into_previous().map(|ephemeral| ephemeral.persistent().into())
            }*/
        }

        impl<S: Spacing$(, $T)?> $name<Range, S$(, $T)?> {
            pub fn bound_type(&self) -> BoundType {
                BoundType::of(self.index.try_into().unwrap())
            }

            pub fn span(&self) -> S {
                self.ephemeral().span()
            }

            pub fn into_range(self) -> (Self, Self) {
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

            pub fn range(&self) -> (MaybeOwned<Self>, MaybeOwned<Self>) {
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
    };
}

index!(Index; <Kind, S: Spacing, T>; Index<Kind, S, T>; Skeleton<Kind, S, T>);

impl<Kind, S: Spacing, T> Index<Kind, S, T> {
    /*pub fn element(&self) -> Ref<T> {
        // self.ephemeral().element()
        // let index = &self.skeleton.borrow().from_persistent[&self.index];
        // index.element()
        // Ref::map(RefCell::borrow(&index.skeleton),
        //          |skeleton| &skeleton.elements[index.index])
        // Ref::map(RefCell::borrow(&self.skeleton),
        //          |skeleton| {
        //              let index = skeleton.from_persistent[&self.index];
        //              &index.element()
                     // &Ref::map(RefCell::borrow(&index.skeleton),
                     //          |skeleton| &skeleton.elements[index.index])
                 // })
    }

    pub fn element_mut(&self) -> RefMut<T> {
        self.ephemeral().element_mut()
    }*/

    pub(crate) fn ephemeral(&self) -> EphemeralIndex<Kind, S, T> {
        self.skeleton.borrow().from_persistent.get(&self.index).cloned()
            .unwrap_or(EphemeralIndex::new(self.skeleton.clone(), self.index as usize))
    }

    pub fn position(&self) -> Position<Kind, S, T> {
        self.ephemeral().position().persistent()
    }
}

impl<Kind, S: Spacing, T> From<EphemeralIndex<Kind, S, T>> for Index<Kind, S, T> {
    fn from(ephemeral: EphemeralIndex<Kind, S, T>) -> Self {
        ephemeral.persistent()
    }
}

impl<Kind, S: Spacing, T> From<Index<Kind, S, T>> for EphemeralIndex<Kind, S, T> {
    fn from(persistent: Index<Kind, S, T>) -> Self {
        persistent.ephemeral()
    }
}

index!(HollowIndex; <Kind, S: Spacing>; HollowIndex<Kind, S>; Skeleton<Kind, S, ()>);

impl<Kind, S: Spacing> HollowIndex<Kind, S> {
    pub(crate) fn ephemeral(&self) -> EphemeralIndex<Kind, S, ()> {
        let index: Index<Kind, S, ()> = self.clone().into();
        index.ephemeral()
    }

    pub fn position(&self) -> HollowPosition<Kind, S> {
        self.ephemeral().position().persistent().into()
    }
}

impl<Kind, S: Spacing> From<Index<Kind, S, ()>> for HollowIndex<Kind, S> {
    fn from(index: Index<Kind, S, ()>) -> Self {
        Self::new(index.skeleton, index.index)
    }
}

impl<Kind, S: Spacing> From<HollowIndex<Kind, S>> for Index<Kind, S, ()> {
    fn from(index: HollowIndex<Kind, S>) -> Self {
        Self::new(index.skeleton, index.index)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BoundType {
    Start,
    End,
}

impl BoundType {
    pub(crate) fn of(index: isize) -> Self {
        match index & 1 {
            0 => Self::Start,
            1 => Self::End,
            _ => unreachable!()
        }
    }
}