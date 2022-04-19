use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use num_traits::zero;

use crate::Spacing;
use crate::traversal::link_index;

struct Node;

struct Range;

pub struct Skeleton<Kind, S: Spacing, T> {
    links: Vec<S>,
    elements: Vec<T>,
    subs: Vec<Self>,
    parent: Option<Weak<RefCell<Self>>>,
    offset: S,
    length: S,
    depth: usize,
    _kind: PhantomData<Kind>,
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

    fn inflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                let link_index = link_index(index >> degree << degree, degree);
                self.links[link_index] += amount;
            }
        }
        self.length += amount
    }

    fn push_link(&mut self) -> usize {
        if (self.links.len() + 1).is_power_of_two() {
            self.links.push(self.length);
        } else {
            self.links.push(zero());
        }
        self.links.len() - 1
    }
}

mod node;

mod range;