use std::cell::RefCell;
use std::rc::Rc;

use maybe_owned::MaybeOwned;

use crate::{BoundType, ElementRef, ElementRefMut, EphemeralIndex, HollowPosition, Position, 
            RangeKind, Skeleton, Spacing};

// TODO rework persistent indices to store not an isize, but a usize with a "generation" counter,
//  basically the "n" in "This index refers to the nth element ever placed at index i."
//  then, rework persistent index handling in before-offset insertion of elements and removal of
//  elements (-> take care that first_persistent_index is always correct!)

macro_rules! index {
    ($name:ident; <Kind, S: Spacing$(, $T:ident)?>; $type:ty; $skeleton:ty) => {
        pub struct $name<Kind, S: Spacing$(, $T)?> {
            pub(crate) skeleton: Rc<RefCell<$skeleton>>,
            pub(crate) index: usize,
            pub(crate) generation: usize,
        }

        impl<Kind, S: Spacing$(, $T)?> Clone for $type {
            fn clone(&self) -> Self {
                Self {
                    skeleton: self.skeleton.clone(),
                    index: self.index,
                    generation: self.generation,
                }
            }
        }

        impl<Kind, S: Spacing$(, $T)?> $type {
            pub(crate) fn new(skeleton: Rc<RefCell<$skeleton>>, index: usize, generation: usize) -> Self {
                Self {
                    skeleton,
                    index,
                    generation,
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

        impl<Kind: RangeKind, S: Spacing$(, $T)?> $name<Kind, S$(, $T)?> {
            pub fn bound_type(&self) -> BoundType {
                BoundType::of(self.index)
            }

            pub fn span(&self) -> S {
                self.ephemeral().span()
            }

            pub fn into_range(self) -> (Self, Self) {
                match self.bound_type() {
                    BoundType::Start => {
                        let end = self.;
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
    pub fn element(&self) -> ElementRef<Kind, S, T> {
        let ephemeral = self.ephemeral();
        ElementRef::new_(ephemeral.skeleton.clone(), ephemeral.index)
    }

    pub fn element_mut(&self) -> ElementRefMut<Kind, S, T> {
        let ephemeral = self.ephemeral();
        ElementRefMut::new_(ephemeral.skeleton.clone(), ephemeral.index)
    }

    pub(crate) fn ephemeral(&self) -> EphemeralIndex<Kind, S, T> {
        self.skeleton.borrow().from_persistent.get().cloned()
            .unwrap_or(EphemeralIndex::new(self.skeleton.clone(), self.index))
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