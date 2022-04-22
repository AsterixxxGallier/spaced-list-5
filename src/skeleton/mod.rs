use std::cell::{RefCell};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use num_traits::zero;

use crate::Spacing;

pub struct Node;

pub struct Range;

pub(crate) struct ParentData<Parent> {
    pub(crate) parent: Weak<RefCell<Parent>>,
    pub(crate) index_in_parent: usize
}

pub struct Skeleton<Kind, S: Spacing, T> {
    links: Vec<S>,
    elements: Vec<T>,
    subs: Vec<Option<Rc<RefCell<Self>>>>,
    parent_data: Option<ParentData<Self>>,
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
    pub(crate) fn new(parent_data: Option<ParentData<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            links: vec![],
            elements: vec![],
            subs: vec![],
            parent_data,
            offset: zero(),
            length: zero(),
            depth: 0,
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
        // ╭───────────────────────────────╮
        // ├───────────────╮               ├───────────────╮
        // ├───────╮       ├───────╮       ├───────╮       │
        // ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮
        // ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵
        // 00000   00010   00100   00110   01000   01010   01100   01110   10000
        //     00001   00011   00101   00111   01001   01011   01101   01111
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
                        index_in_parent: index
                    }))).clone()
        }
    }
}

mod flate;

mod node;

mod range;

pub mod traversal;

pub mod position;