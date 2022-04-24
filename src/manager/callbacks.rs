use std::cell::Cell;
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
    pub(super) indices: Cell<Vec<Box<dyn Fn(IndexChange<S, T>)>>>,
    pub(super) positions: Cell<Vec<Box<dyn Fn(SpacingChange<S>)>>>,
    pub(super) insertions: Cell<Vec<Box<dyn Fn(Insertion<S, T>)>>>,
}