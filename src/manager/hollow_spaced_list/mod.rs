use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{HollowPosition, HollowSpacedList, Node, Spacing};
use self::handles::{HollowInsertionsHandle, HollowPositionsHandle};
use self::locks::{HollowInsertionsLock, HollowPositionsLock};

pub mod locks;
pub mod handles;

struct HollowLockedPosition<S: Spacing> {
    position: HollowPosition<Node, S>,
    lock: HollowPositionsLock<S>
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
    pub fn new(list: HollowSpacedList<S>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: HollowLocks::default(),
        }))
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
}
