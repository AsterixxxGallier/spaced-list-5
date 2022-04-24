use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::{Node, Position, SpacedList, Spacing};
use crate::manager::callbacks::Callbacks;

// TODO make this entire module work for all SpacedLists and also work in the first place

struct Locks {
    // -1: indices might change
    // > 0: indices may not change (structure must be preserved)
    indices: Cell<isize>,

    // -1: elements might be moved
    // > 0: elements may not be moved (spacing must be preserved)
    positions: Cell<isize>,

    // -1: elements might be added
    // > 0: elements may not be added
    insertions: Cell<isize>,

    // -1: elements might be removed
    // > 0: elements may not be removed TODO implement the ability to remove elements
    // deletions: Cell<isize>,

    // -1: elements might be mutated
    // > 0: elements may not be mutated (values must be preserved)
    values: Cell<isize>,
}

mod callbacks;

pub struct Manager<S: Spacing, T> {
    list: SpacedList<S, T>,
    locks: Locks,
    callbacks: Callbacks<S, T>,
}

mod locks;

mod handles;