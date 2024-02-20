use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{Node, HollowPosition, Spacing, HollowSpacedList};
use crate::manager::{HollowInsertionsHandle, HollowPositionsHandle,
                     HollowInsertionsLock, HollowPositionsLock};

pub mod locks;
pub mod handles;

pub struct HollowLockedPosition<S: Spacing> {
    position: HollowPosition<Node, S>,
    lock: HollowPositionsLock<S>,
}

#[derive(Default)]
struct HollowLocks {
    // -1: elements might be moved
    // > 0: elements may not be moved (spacing must be preserved)
    positions: Cell<isize>,

    // -1: elements might be added
    // > 0: elements may not be added
    insertions: Cell<isize>,

    // -1: elements might be removed
    // > 0: elements may not be removed TODO implement the ability to remove elements
    // deletions: Cell<isize>,
}

pub struct HollowManager<S: Spacing> {
    list: HollowSpacedList<S>,
    locks: HollowLocks,
}

impl<S: Spacing> HollowManager<S> {
    #[must_use]
     pub fn new(list: HollowSpacedList<S>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: HollowLocks::default(),
        }))
    }

    pub(crate) fn lock(this: Rc<RefCell<Self>>, position: HollowPosition<Node, S>) -> HollowLockedPosition<S> {
        HollowLockedPosition {
            position,
            lock: HollowManager::positions_lock(this),
        }
    }

    pub fn positions_lock(this: Rc<RefCell<Self>>) -> HollowPositionsLock<S> {
        HollowPositionsLock::new(this)
    }

    pub fn insertions_lock(this: Rc<RefCell<Self>>) -> HollowInsertionsLock<S> {
        HollowInsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> HollowDeletionsLock<S> {
        HollowDeletionsLock::new(this)
    }*/

    pub fn positions_handle(this: Rc<RefCell<Self>>) -> HollowPositionsHandle<S> {
        HollowPositionsHandle::new(this)
    }

    pub fn insertions_handle(this: Rc<RefCell<Self>>) -> HollowInsertionsHandle<S> {
        HollowInsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> HollowDeletionsHandle<S> {
        HollowDeletionsHandle::new(this)
    }*/

    pub fn before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowLockedPosition<S>> {
        this.borrow().list.before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowLockedPosition<S>> {
        this.borrow().list.at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn at(this: Rc<RefCell<Self>>, position: S) -> Option<HollowLockedPosition<S>> {
        this.borrow().list.at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowLockedPosition<S>> {
        this.borrow().list.at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowLockedPosition<S>> {
        this.borrow().list.after(position)
            .map(|position| Self::lock(this.clone(), position))
    }
}
