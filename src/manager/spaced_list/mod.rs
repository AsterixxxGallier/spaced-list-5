use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{SpacedList, Spacing};
use self::callback_locks::{IndicesCallbackLock, InsertionsCallbackLock, PositionsCallbackLock};
use self::callbacks::{Callbacks, IndexChange, Insertion, SpacingChange};
use self::handles::{IndicesHandle, InsertionsHandle, PositionsHandle, ValuesHandle};
use self::locks::{IndicesLock, InsertionsLock, PositionsLock, ValuesLock};

#[derive(Default)]
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

pub struct Manager<'callbacks, S: Spacing, T> {
    list: SpacedList<S, T>,
    locks: Locks,
    callbacks: Callbacks<'callbacks, S, T>,
}

impl<'callbacks, S: Spacing, T> Manager<'callbacks, S, T> {
    fn new(list: SpacedList<S, T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: Locks::default(),
            callbacks: Callbacks::default(),
        }))
    }

    fn indices_lock(this: Rc<RefCell<Self>>) -> IndicesLock<'callbacks, S, T> {
        IndicesLock::new(this)
    }

    fn positions_lock(this: Rc<RefCell<Self>>) -> PositionsLock<'callbacks, S, T> {
        PositionsLock::new(this)
    }

    fn insertions_lock(this: Rc<RefCell<Self>>) -> InsertionsLock<'callbacks, S, T> {
        InsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> DeletionsLock<'callbacks, S, T> {
        DeletionsLock::new(this)
    }*/

    fn values_lock(this: Rc<RefCell<Self>>) -> ValuesLock<'callbacks, S, T> {
        ValuesLock::new(this)
    }

    fn indices_callback<F>(this: Rc<RefCell<Self>>, callback: F)
        -> IndicesCallbackLock<'callbacks, S, T, F>
        where F: Fn(IndexChange<S, T>) {
        IndicesCallbackLock::new(this, callback)
    }

    fn positions_callback<F>(this: Rc<RefCell<Self>>, callback: F)
        -> PositionsCallbackLock<'callbacks, S, T, F>
        where F: Fn(SpacingChange<S>) {
        PositionsCallbackLock::new(this, callback)
    }

    fn insertions_callback<F>(this: Rc<RefCell<Self>>, callback: F)
        -> InsertionsCallbackLock<'callbacks, S, T, F>
        where F: Fn(Insertion<S, T>) {
        InsertionsCallbackLock::new(this, callback)
    }

    fn indices_handle(this: Rc<RefCell<Self>>) -> IndicesHandle<'callbacks, S, T> {
        IndicesHandle::new(this)
    }

    fn positions_handle(this: Rc<RefCell<Self>>) -> PositionsHandle<'callbacks, S, T> {
        PositionsHandle::new(this)
    }

    fn insertions_handle(this: Rc<RefCell<Self>>) -> InsertionsHandle<'callbacks, S, T> {
        InsertionsHandle::new(this)
    }

    /*fn lock_deletions(this: Rc<RefCell<Self>>) -> DeletionsHandle<'callbacks, S, T> {
        DeletionsHandle::new(this)
    }*/

    fn lock_values(this: Rc<RefCell<Self>>) -> ValuesHandle<'callbacks, S, T> {
        ValuesHandle::new(this)
    }
}

mod locks;

mod callback_locks;

mod handles;