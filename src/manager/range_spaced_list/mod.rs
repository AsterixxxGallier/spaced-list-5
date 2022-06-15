use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{Position, Range, RangeSpacedList, Spacing};
use self::handles::{RangeInsertionsHandle, RangePositionsHandle, RangeValuesHandle};
use self::locks::{RangeInsertionsLock, RangePositionsLock, RangeValuesLock};

pub mod locks;
pub mod handles;

pub struct RangeLockedPosition<S: Spacing, T> {
    position: Position<Range, S, T>,
    lock: RangePositionsLock<S, T>
}

#[derive(Default)]
struct RangeLocks {
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

pub struct RangeManager<S: Spacing, T> {
    list: RangeSpacedList<S, T>,
    locks: RangeLocks,
}

impl<S: Spacing, T> RangeManager<S, T> {
    pub fn new(list: RangeSpacedList<S, T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: RangeLocks::default(),
        }))
    }

    pub(crate) fn lock(this: Rc<RefCell<Self>>, position: Position<Range, S, T>) -> RangeLockedPosition<S, T> {
        RangeLockedPosition {
            position,
            lock: RangeManager::positions_lock(this)
        }
    }

    pub fn positions_lock(this: Rc<RefCell<Self>>) -> RangePositionsLock<S, T> {
        RangePositionsLock::new(this)
    }

    pub fn insertions_lock(this: Rc<RefCell<Self>>) -> RangeInsertionsLock<S, T> {
        RangeInsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> RangeDeletionsLock<S, T> {
        RangeDeletionsLock::new(this)
    }*/

    pub fn values_lock(this: Rc<RefCell<Self>>) -> RangeValuesLock<S, T> {
        RangeValuesLock::new(this)
    }

    pub fn positions_handle(this: Rc<RefCell<Self>>) -> RangePositionsHandle<S, T> {
        RangePositionsHandle::new(this)
    }

    pub fn insertions_handle(this: Rc<RefCell<Self>>) -> RangeInsertionsHandle<S, T> {
        RangeInsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> RangeDeletionsHandle<S, T> {
        RangeDeletionsHandle::new(this)
    }*/

    pub fn values_handle(this: Rc<RefCell<Self>>) -> RangeValuesHandle<S, T> {
        RangeValuesHandle::new(this)
    }
}
