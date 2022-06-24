use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use nohash_hasher::IntMap;
use num_traits::zero;

use crate::Spacing;
use crate::skeleton::index::{EphemeralIndex, Index};

pub struct Node;

pub struct Range;

/// Third kind: NestedRange
/// Requirements:
/// - normal ranges
/// - nested ranges inside them
///
/// simple solution: store sublists per range

pub(crate) struct ParentData<Parent> {
    pub(crate) parent: Weak<RefCell<Parent>>,
    pub(crate) index_in_parent: usize,
}

pub(crate) struct Skeleton<Kind, S: Spacing, T> {
    links: Vec<S>,
    elements: Vec<T>,
    subs: Vec<Option<Rc<RefCell<Self>>>>,
    parent_data: Option<ParentData<Self>>,
    offset: S,
    length: S,
    depth: usize,
    first_persistent_index: isize,
    from_persistent: IntMap<isize, EphemeralIndex<Kind, S, T>>,
    into_persistent: IntMap<usize, Index<Kind, S, T>>,
    _kind: PhantomData<Kind>,
}

#[inline(always)]
pub(crate) const fn link_index(index: usize, degree: usize) -> usize {
    index | ((1 << degree) - 1)
}

#[inline(always)]
pub(crate) const fn relative_depth(index: usize, size: usize) -> usize {
    (usize::BITS - (size ^ index).leading_zeros()) as usize
}

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    pub(crate) fn new(parent_data: Option<ParentData<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            links: vec![],
            elements: vec![],
            subs: vec![],
            parent_data,
            offset: zero(),
            length: zero(),
            depth: 0,
            first_persistent_index: 0,
            from_persistent: IntMap::default(),
            into_persistent: IntMap::default(),
            _kind: PhantomData::<Kind>,
        }))
    }

    fn link_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.links.len()
    }

    fn link(&self, index: usize) -> S {
        let mut length = self.links[index];
        for degree in 0..index.trailing_ones() {
            length -= self.links[index - (1 << degree)];
        }
        length
    }

    fn push_link(&mut self) -> usize {
        let mut length = S::zero();
        let index = self.links.len();
        for degree in 0..index.trailing_ones() {
            length += self.links[index - (1 << degree)];
        }
        self.links.push(length);
        if self.links.len().is_power_of_two() {
            self.depth += 1;
        }
        self.subs.push(None);
        index
    }

    pub fn offset(&self) -> S {
        self.offset
    }

    pub fn length(&self) -> S {
        self.length
    }

    pub fn last_position(&self) -> S {
        self.offset + self.length
    }

    fn sub(&self, index: usize) -> Option<Rc<RefCell<Self>>> {
        self.subs.get(index).cloned().flatten()
    }

    fn ensure_sub(this: Rc<RefCell<Self>>, index: usize) -> Rc<RefCell<Self>> {
        match &mut this.borrow_mut().subs[index] {
            Some(sub) => sub.clone(),
            none =>
                none.insert(Skeleton::new(Some(
                    ParentData {
                        parent: Rc::downgrade(&this),
                        index_in_parent: index,
                    }))).clone()
        }
    }
}

mod flate;

mod node;

mod range;

pub mod traversal;

pub mod position;

pub mod index;