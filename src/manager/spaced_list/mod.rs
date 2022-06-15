use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{Node, Position, SpacedList, Spacing};

use self::handles::{InsertionsHandle, PositionsHandle, ValuesHandle};
use self::locks::{InsertionsLock, PositionsLock, ValuesLock};

pub mod locks;
pub mod handles;

pub struct LockedPosition<S: Spacing, T> {
    position: Position<Node, S, T>,
    lock: PositionsLock<S, T>,
}

#[derive(Default)]
struct Locks {
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

pub struct Manager<S: Spacing, T> {
    list: SpacedList<S, T>,
    locks: Locks,
}

impl<S: Spacing, T> Manager<S, T> {
    pub fn new(list: SpacedList<S, T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: Locks::default(),
        }))
    }

    pub(crate) fn lock(this: Rc<RefCell<Self>>, position: Position<Node, S, T>) -> LockedPosition<S, T> {
        LockedPosition {
            position,
            lock: Manager::positions_lock(this)
        }
    }

    pub fn positions_lock(this: Rc<RefCell<Self>>) -> PositionsLock<S, T> {
        PositionsLock::new(this)
    }

    pub fn insertions_lock(this: Rc<RefCell<Self>>) -> InsertionsLock<S, T> {
        InsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> DeletionsLock<S, T> {
        DeletionsLock::new(this)
    }*/

    pub fn values_lock(this: Rc<RefCell<Self>>) -> ValuesLock<S, T> {
        ValuesLock::new(this)
    }

    pub fn positions_handle(this: Rc<RefCell<Self>>) -> PositionsHandle<S, T> {
        PositionsHandle::new(this)
    }

    pub fn insertions_handle(this: Rc<RefCell<Self>>) -> InsertionsHandle<S, T> {
        InsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> DeletionsHandle<S, T> {
        DeletionsHandle::new(this)
    }*/

    pub fn values_handle(this: Rc<RefCell<Self>>) -> ValuesHandle<S, T> {
        ValuesHandle::new(this)
    }

    pub fn before(this: Rc<RefCell<Self>>, position: S) -> Option<LockedPosition<S, T>> {
        this.borrow().list.before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<LockedPosition<S, T>> {
        this.borrow().list.at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn at(this: Rc<RefCell<Self>>, position: S) -> Option<LockedPosition<S, T>> {
        this.borrow().list.at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<LockedPosition<S, T>> {
        this.borrow().list.at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn after(this: Rc<RefCell<Self>>, position: S) -> Option<LockedPosition<S, T>> {
        this.borrow().list.after(position)
            .map(|position| Self::lock(this.clone(), position))
    }
}
