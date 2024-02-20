use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Debug;

use itertools::Itertools;
use maybe_owned::MaybeOwned;

use crate::{BackwardsIter, BoundType, ElementRef, ElementRefMut, EphemeralPosition, ForwardsIter,
            HollowIndex, Index, RangeKind, Skeleton, Spacing};

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

        impl<Kind: RangeKind, S: Spacing$(, $T)?> $name<Kind, S$(, $T)?> {
            pub fn bound_type(&self) -> BoundType {
                BoundType::of_signed(self.index)
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

        impl<Kind, S: Spacing + Display$(, $T)?> Display for $type {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(concat!(stringify!($name), " { "))?;
                self.position.fmt(f)?;
                f.write_str(" }")?;
                Ok(())
            }
        }

        impl<Kind, S: Spacing + Debug$(, $T)?> Debug for $type {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("position", &self.position)
                    .field("index", &self.index)
                    .finish()
            }
        }
    };
}

position!(Position; <Kind, S: Spacing, T>; Position<Kind, S, T>; Skeleton<Kind, S, T>);

impl<Kind, S: Spacing, T> Position<Kind, S, T> {
    pub fn element(&self) -> ElementRef<Kind, S, T> {
        let ephemeral = self.ephemeral();
        ElementRef::new_(ephemeral.skeleton.clone(), ephemeral.index)
    }

    pub fn element_mut(&self) -> ElementRefMut<Kind, S, T> {
        let ephemeral = self.ephemeral();
        ElementRefMut::new_(ephemeral.skeleton.clone(), ephemeral.index)
    }

    pub(crate) fn ephemeral(&self) -> EphemeralPosition<Kind, S, T> {
        self.skeleton.borrow().from_persistent.get(&self.index).cloned()
            .map_or(
                EphemeralPosition::new(self.skeleton.clone(), self.index as usize, self.position),
                |index| EphemeralPosition::new(index.skeleton, index.index, self.position),
            )
    }

    pub fn into_index(self) -> Index<Kind, S, T> {
        Index::new(self.skeleton, self.index)
    }

    pub fn index(&self) -> Index<Kind, S, T> {
        Index::new(self.skeleton.clone(), self.index)
    }
}

impl<Kind, S: Spacing, T> From<EphemeralPosition<Kind, S, T>> for Position<Kind, S, T> {
    fn from(ephemeral: EphemeralPosition<Kind, S, T>) -> Self {
        ephemeral.persistent()
    }
}

position!(HollowPosition; <Kind, S: Spacing>; HollowPosition<Kind, S>; Skeleton<Kind, S, ()>);

impl<Kind, S: Spacing> HollowPosition<Kind, S> {
    pub(crate) fn ephemeral(&self) -> EphemeralPosition<Kind, S, ()> {
        let position: Position<Kind, S, ()> = self.clone().into();
        position.ephemeral()
    }

    pub fn into_index(self) -> HollowIndex<Kind, S> {
        HollowIndex::new(self.skeleton, self.index)
    }

    pub fn index(&self) -> HollowIndex<Kind, S> {
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