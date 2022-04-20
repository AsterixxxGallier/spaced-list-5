use std::cell::{RefCell};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use num_traits::zero;

use crate::Spacing;

pub(crate) struct Node;

pub(crate) struct Range;

pub struct Skeleton<Kind, S: Spacing, T> {
    links: Vec<S>,
    elements: Vec<T>,
    subs: Vec<Option<Rc<RefCell<Self>>>>,
    parent: Option<Weak<RefCell<Self>>>,
    offset: S,
    length: S,
    depth: usize,
    _kind: PhantomData<Kind>,
}

#[inline(always)]
pub(crate) const fn link_index(index: usize, degree: usize) -> usize {
    index | ((1 << degree) - 1)
}

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            links: vec![],
            elements: vec![],
            subs: vec![],
            parent: None,
            offset: zero(),
            length: zero(),
            depth: 0,
            _kind: PhantomData::<Kind>,
        }))
    }

    fn link_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.links.len()
    }

    fn inflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                self.links[link_index(index, degree)] += amount;
            }
        }
        self.length += amount
    }

    fn inflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        self.inflate_unchecked(index, amount)
    }

    fn deflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                self.links[link_index(index, degree)] -= amount;
            }
        }
        self.length -= amount
    }

    fn deflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero(), "Cannot deflate by negative amount, explicitly inflate for that");
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                assert!(self.link(link_index(index, degree)) >= amount,
                        "Deflating at this index would deflate a link below zero");
            }
        }
        self.inflate_unchecked(index, amount)
    }

    fn link(&self, index: usize) -> S {
        let mut length = self.links[index];
        for degree in 0..index.trailing_ones() {
            length -= self.links[index - (1 << degree)];
        }
        length
    }

    fn push_link(&mut self) -> usize {
        if (self.links.len() + 1).is_power_of_two() {
            self.links.push(self.length);
        } else {
            self.links.push(zero());
        }
        self.links.len() - 1
    }

    fn last_position(&self) -> S {
        self.offset + self.length
    }

    fn sub(&self, index: usize) -> Option<Rc<RefCell<Self>>> {
        self.subs.get(index).cloned().flatten()
    }
}

mod node;

mod range;

mod traversal;