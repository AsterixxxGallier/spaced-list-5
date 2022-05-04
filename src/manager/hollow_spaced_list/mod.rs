use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{HollowSpacedList, Spacing};
use self::handles::{HollowIndicesHandle, HollowInsertionsHandle, HollowPositionsHandle};
use self::locks::{HollowIndicesLock, HollowInsertionsLock, HollowPositionsLock};

pub mod locks;
pub mod handles;

#[derive(Default)]
struct HollowLocks {
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
}

pub struct HollowManager<S: Spacing> {
    list: HollowSpacedList<S>,
    locks: HollowLocks,
}

impl<S: Spacing> HollowManager<S> {
    fn new(list: HollowSpacedList<S>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: HollowLocks::default(),
        }))
    }

    fn indices_lock(this: Rc<RefCell<Self>>) -> HollowIndicesLock<S> {
        HollowIndicesLock::new(this)
    }

    fn positions_lock(this: Rc<RefCell<Self>>) -> HollowPositionsLock<S> {
        HollowPositionsLock::new(this)
    }

    fn insertions_lock(this: Rc<RefCell<Self>>) -> HollowInsertionsLock<S> {
        HollowInsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> HollowDeletionsLock<S> {
        HollowDeletionsLock::new(this)
    }*/

    fn indices_handle(this: Rc<RefCell<Self>>) -> HollowIndicesHandle<S> {
        HollowIndicesHandle::new(this)
    }

    fn positions_handle(this: Rc<RefCell<Self>>) -> HollowPositionsHandle<S> {
        HollowPositionsHandle::new(this)
    }

    fn insertions_handle(this: Rc<RefCell<Self>>) -> HollowInsertionsHandle<S> {
        HollowInsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> HollowDeletionsHandle<S> {
        HollowDeletionsHandle::new(this)
    }*/
}
