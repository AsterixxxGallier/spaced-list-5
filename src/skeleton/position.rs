use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use itertools::Itertools;
use maybe_owned::MaybeOwned;

use crate::{BackwardsIter, ForwardsIter, ParentData, Spacing};
use crate::skeleton::{Range, Skeleton};

macro_rules! position {
    ($name:ident; <Kind, S: Spacing$(, $T:ident)?>; $type:ty; $skeleton:ty) => {
        pub struct $name<Kind, S: Spacing$(, $T)?> {
            pub(crate) skeleton: Rc<RefCell<$skeleton>>,
            // TODO implement persistent indices or something
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
                /*if self.index == self.skeleton.borrow().links.len() as isize {
                    if let Some(ParentData { parent, index_in_parent }) =
                    &self.skeleton.clone().borrow().parent_data {
                        let parent = parent.upgrade().unwrap();
                        let position = self.position
                            - self.skeleton.borrow().last_position()
                            + parent.borrow().link(*index_in_parent);
                        Some(Self {
                            skeleton: parent,
                            index: index_in_parent as isize + 1,
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
                }*/
            }

            pub fn into_previous(self) -> Option<Self> {
                self.ephemeral().into_previous().map(|ephemeral| ephemeral.persistent().into())
                /*if self.index == 0 {
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
                }*/
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

// TODO make this crate-private
// position!(EphemeralPosition; <Kind, S: Spacing, T>; EphemeralPosition<Kind, S, T>; Skeleton<Kind, S, T>; usize; 0);
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

    pub(crate) fn persistent(&self) -> Position<Kind, S, T> {
        self.skeleton.borrow().into_persistent.get(&self.index).cloned()
            .unwrap_or(Position::new(self.skeleton.clone(), self.index as isize, self.position))
    }

    pub fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index])
    }

    pub fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index])
    }

    pub fn into_next(self) -> Option<Self> {
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

    pub fn into_previous(self) -> Option<Self> {
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
    pub fn bound_type(&self) -> BoundType {
        BoundType::of(self.index.try_into().unwrap())
    }

    pub fn span(&self) -> S {
        self.skeleton.borrow().links[self.index & !1]
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

// skeleton.borrow().first_persistent_index

position!(Position; <Kind, S: Spacing, T>; Position<Kind, S, T>; Skeleton<Kind, S, T>);

/*pub struct Position<Kind, S: Spacing, T> {
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
        let index = 0;
        let position = skeleton.borrow().offset;
        Self {
            skeleton,
            index,
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

    pub fn iter_next(&self) -> impl Iterator<Item=Self> {
        ForwardsIter::from(self.clone().into()).map_into()
    }

    pub fn into_iter_next(self) -> impl Iterator<Item=Self> {
        ForwardsIter::from(self.into()).map_into()
    }

    pub fn iter_previous(&self) -> impl Iterator<Item=Self> {
        BackwardsIter::from(self.clone().into()).map_into()
    }

    pub fn into_iter_previous(self) -> impl Iterator<Item=Self> {
        BackwardsIter::from(self.into()).map_into()
    }

    pub fn into_next(self) -> Option<Self> {
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

    pub fn into_previous(self) -> Option<Self> {
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

impl<S: Spacing, T> Position<Range, S, T> {
    pub fn bound_type(&self) -> BoundType {
        BoundType::of(self.index.try_into().unwrap())
    }

    pub fn span(&self) -> S {
        self.skeleton.borrow().links[self.index & !1]
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
}*/

impl<Kind, S: Spacing, T> Position<Kind, S, T> {
    /*pub fn element(&self) -> Ref<T> {
        // self.ephemeral().element()
        // let position = &self.skeleton.borrow().from_persistent[&self.index];
        // position.element()
        // Ref::map(RefCell::borrow(&position.skeleton),
        //          |skeleton| &skeleton.elements[position.index])
        // Ref::map(RefCell::borrow(&self.skeleton),
        //          |skeleton| {
        //              let position = skeleton.from_persistent[&self.index];
        //              &position.element()
                     // &Ref::map(RefCell::borrow(&position.skeleton),
                     //          |skeleton| &skeleton.elements[position.index])
                 // })
    }

    pub fn element_mut(&self) -> RefMut<T> {
        self.ephemeral().element_mut()
    }*/

    pub(crate) fn ephemeral(&self) -> EphemeralPosition<Kind, S, T> {
        self.skeleton.borrow().from_persistent.get(&self.index).cloned()
            .unwrap_or(EphemeralPosition::new(self.skeleton.clone(), self.index as usize, self.position))
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