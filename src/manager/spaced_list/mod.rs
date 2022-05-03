use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{SpacedList, Spacing};
use crate::manager::handles::{IndicesHandle, InsertionsHandle, PositionsHandle, ValuesHandle};
use crate::manager::locks::{IndicesLock, InsertionsLock, PositionsLock, ValuesLock};

#[derive(Default)]
struct SpacedListLocks {
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

pub struct SpacedListManager<S: Spacing, T> {
    list: SpacedList<S, T>,
    locks: SpacedListLocks,
}

impl<S: Spacing, T> SpacedListManager<S, T> {
    fn new(list: SpacedList<S, T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: SpacedListLocks::default(),
        }))
    }

    fn indices_lock(this: Rc<RefCell<Self>>) -> IndicesLock<S, T> {
        IndicesLock::new(this)
    }

    fn positions_lock(this: Rc<RefCell<Self>>) -> PositionsLock<S, T> {
        PositionsLock::new(this)
    }

    fn insertions_lock(this: Rc<RefCell<Self>>) -> InsertionsLock<S, T> {
        InsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> DeletionsLock<S, T> {
        DeletionsLock::new(this)
    }*/

    fn values_lock(this: Rc<RefCell<Self>>) -> ValuesLock<S, T> {
        ValuesLock::new(this)
    }

    fn indices_handle(this: Rc<RefCell<Self>>) -> IndicesHandle<S, T> {
        IndicesHandle::new(this)
    }

    fn positions_handle(this: Rc<RefCell<Self>>) -> PositionsHandle<S, T> {
        PositionsHandle::new(this)
    }

    fn insertions_handle(this: Rc<RefCell<Self>>) -> InsertionsHandle<S, T> {
        InsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> DeletionsHandle<S, T> {
        DeletionsHandle::new(this)
    }*/

    fn values_handle(this: Rc<RefCell<Self>>) -> ValuesHandle<S, T> {
        ValuesHandle::new(this)
    }
}

