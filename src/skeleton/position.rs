use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr::null;
use std::rc::Rc;

use itertools::Itertools;
use maybe_owned::MaybeOwned;

use crate::{BackwardsIter, ForwardsIter, ParentData, Spacing};
use crate::skeleton::{Range, Skeleton};
use crate::skeleton::index::{EphemeralIndex, HollowIndex, Index};

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

impl<S: Spacing, T> EphemeralPosition<Range, S, T> {
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

/*pub struct ElementRef<'a, Kind, S: Spacing + 'a, T: 'a> {
    skeleton: Ref<'a, Skeleton<Kind, S, T>>,
    index: usize,
}

impl<'a, Kind, S: Spacing + 'a, T: 'a> ElementRef<'a, Kind, S, T> {
    fn new(skeleton: Ref<'a, Skeleton<Kind, S, T>>, index: usize) -> Self {
        Self {
            skeleton,
            index,
        }
    }

    fn borrow(&self) -> Ref<'a, T> {
        Ref::map(Ref::clone(&self.skeleton), |x| &x.elements[self.index])
    }
}*/

pub struct ElementRef<Kind, S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,
    index: usize,
}

impl<Kind, S: Spacing, T> ElementRef<Kind, S, T> {
    fn new(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>, index: usize) -> Self {
        Self {
            skeleton,
            index,
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.skeleton.borrow(), |x| &x.elements[self.index])
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut::map(self.skeleton.borrow_mut(), |x| &mut x.elements[self.index])
    }
}

// impl<'a, Kind, S: Spacing + 'a, T: 'a> Deref for ElementRef<'a, Kind, S, T> {
//     type Target = Ref<'a, T>;
//
//     fn deref(&self) -> &Self::Target {
//         let borrow = self.skeleton.borrow();
//         &Ref::map(borrow,
//                   |skeleton| &skeleton.elements[self.index])
//     }
// }

/*pub struct ElementRefMut<Kind, S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,
    index: usize,
}

impl<Kind, S: Spacing, T> Deref for ElementRefMut<Kind, S, T> {
    type Target = RefMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &RefMut::map(self.skeleton.borrow_mut(),
                     |skeleton| &mut skeleton.elements[self.index])
    }
}

impl<Kind, S: Spacing, T> DerefMut for ElementRefMut<Kind, S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.reference.assume_init() }
    }
}*/

macro_rules! position {
    ($name:ident; <Kind, S: Spacing$(, $T:ident)?>; $type:ty; $skeleton:ty) => {
        pub struct $name<Kind, S: Spacing$(, $T)?> {
            pub(crate) skeleton: Rc<RefCell<$skeleton>>,
            pub(crate) index: isize,
            pub(crate) position: S,
        }

        impl<Kind, S: Spacing$(, $T)?> Clone for $type {
            fn clone(&self) -> Self {
                Self {
                    skeleton: self.skeleton.clone(),
                    index: self.index,
                    position: self.position,
                }
            }
        }

        impl<Kind, S: Spacing$(, $T)?> $type {
            pub(crate) fn new(skeleton: Rc<RefCell<$skeleton>>, index: isize, position: S) -> Self {
                Self {
                    skeleton,
                    index,
                    position,
                }
            }

            pub(crate) fn at_start(skeleton: Rc<RefCell<$skeleton>>) -> Self {
                let index = skeleton.borrow().first_persistent_index;
                let position = skeleton.borrow().offset;
                Self {
                    skeleton,
                    index,
                    position,
                }
            }

            pub(crate) fn at_end(skeleton: Rc<RefCell<$skeleton>>) -> Self {
                let index = (skeleton.borrow().elements.len() - 1) as isize;
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

            pub fn iter_next(&self) -> impl Iterator<Item = Self> {
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
            }
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

            pub fn range(&self) -> (MaybeOwned<Self>, MaybeOwned<Self>) {
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
    };
}

position!(Position; <Kind, S: Spacing, T>; Position<Kind, S, T>; Skeleton<Kind, S, T>);

impl<Kind, S: Spacing, T> Position<Kind, S, T> {
    pub fn element(&self) -> ElementRef<Kind, S, T> {
        match self.skeleton.borrow().from_persistent.get(&self.index) {
            Some(EphemeralIndex { skeleton, index }) => {
                ElementRef::new(skeleton.clone(), *index)
            }
            None => {
                ElementRef::new(self.skeleton.clone(), self.index as usize)
            }
        }
    }

    pub(crate) fn ephemeral(&self) -> EphemeralPosition<Kind, S, T> {
        self.skeleton.borrow().from_persistent.get(&self.index).cloned()
            .map_or(
                EphemeralPosition::new(self.skeleton.clone(), self.index as usize, self.position),
                |index| EphemeralPosition::new(index.skeleton, index.index, self.position),
            )
    }

    pub(crate) fn into_index(self) -> Index<Kind, S, T> {
        Index::new(self.skeleton, self.index)
    }

    pub(crate) fn index(&self) -> Index<Kind, S, T> {
        Index::new(self.skeleton.clone(), self.index)
    }
}

impl<Kind, S: Spacing, T> From<EphemeralPosition<Kind, S, T>> for Position<Kind, S, T> {
    fn from(ephemeral: EphemeralPosition<Kind, S, T>) -> Self {
        ephemeral.persistent()
    }
}

impl<Kind, S: Spacing, T> From<Position<Kind, S, T>> for EphemeralPosition<Kind, S, T> {
    fn from(persistent: Position<Kind, S, T>) -> Self {
        persistent.ephemeral()
    }
}

position!(HollowPosition; <Kind, S: Spacing>; HollowPosition<Kind, S>; Skeleton<Kind, S, ()>);

impl<Kind, S: Spacing> HollowPosition<Kind, S> {
    pub(crate) fn ephemeral(&self) -> EphemeralPosition<Kind, S, ()> {
        let position: Position<Kind, S, ()> = self.clone().into();
        position.ephemeral()
    }

    pub(crate) fn into_index(self) -> HollowIndex<Kind, S> {
        HollowIndex::new(self.skeleton, self.index)
    }

    pub(crate) fn index(&self) -> HollowIndex<Kind, S> {
        HollowIndex::new(self.skeleton.clone(), self.index)
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