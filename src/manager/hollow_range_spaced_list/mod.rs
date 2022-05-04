use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{HollowRangeSpacedList, Spacing};
use self::handles::{HollowRangeInsertionsHandle, HollowRangePositionsHandle};
use self::locks::{HollowRangeInsertionsLock, HollowRangePositionsLock};

pub mod locks;
pub mod handles;

#[derive(Default)]
struct HollowRangeLocks {
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

pub struct HollowRangeManager<S: Spacing> {
    list: HollowRangeSpacedList<S>,
    locks: HollowRangeLocks,
}

impl<S: Spacing> HollowRangeManager<S> {
    pub fn new(list: HollowRangeSpacedList<S>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: HollowRangeLocks::default(),
        }))
    }

    pub fn positions_lock(this: Rc<RefCell<Self>>) -> HollowRangePositionsLock<S> {
        HollowRangePositionsLock::new(this)
    }

    pub fn insertions_lock(this: Rc<RefCell<Self>>) -> HollowRangeInsertionsLock<S> {
        HollowRangeInsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> HollowRangeDeletionsLock<S> {
        HollowRangeDeletionsLock::new(this)
    }*/

    pub fn positions_handle(this: Rc<RefCell<Self>>) -> HollowRangePositionsHandle<S> {
        HollowRangePositionsHandle::new(this)
    }

    pub fn insertions_handle(this: Rc<RefCell<Self>>) -> HollowRangeInsertionsHandle<S> {
        HollowRangeInsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> HollowRangeDeletionsHandle<S> {
        HollowRangeDeletionsHandle::new(this)
    }*/
}
