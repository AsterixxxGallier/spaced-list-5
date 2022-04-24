use std::cell::{Cell, RefCell};
use slab::Slab;
use crate::{Spacing, Position, Node};

pub enum SpacingChange<S: Spacing> {
    IncreaseSpacingAfter {
        position: S,
        amount: S
    },
    IncreaseSpacingBefore {
        position: S,
        amount: S
    },
    DecreaseSpacingAfter {
        position: S,
        amount: S
    },
    DecreaseSpacingBefore {
        position: S,
        amount: S
    },
}

pub struct Insertion<S: Spacing, T> {
    position: S,
    value: T
}

pub enum IndexChange<S: Spacing, T> {
    OffsetSwapAround {
        old_position: Position<Node, S, T>,
        new_position: Position<Node, S, T>,
    }
}

pub(super) struct Callbacks<S: Spacing, T> {
    pub(super) indices: RefCell<Slab<&dyn Fn(IndexChange<S, T>)>>,
    pub(super) positions: RefCell<Slab<&dyn Fn(SpacingChange<S>)>>,
    pub(super) insertions: RefCell<Slab<&dyn Fn(Insertion<S, T>)>>,
}

impl<S: Spacing, T> Default for Callbacks<S, T> {
    fn default() -> Self {
        Self {
            indices: RefCell::new(Slab::new()),
            positions: RefCell::new(Slab::new()),
            insertions: RefCell::new(Slab::new()),
        }
    }
}