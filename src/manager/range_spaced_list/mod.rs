use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{RangeSpacedList, Spacing};
use self::handles::{RangeIndicesHandle, RangeInsertionsHandle, RangePositionsHandle, RangeValuesHandle};
use self::locks::{RangeIndicesLock, RangeInsertionsLock, RangePositionsLock, RangeValuesLock};

pub mod locks;
pub mod handles;

#[derive(Default)]
struct RangeLocks {
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

pub struct RangeManager<S: Spacing, T> {
    list: RangeSpacedList<S, T>,
    locks: RangeLocks,
}

impl<S: Spacing, T> RangeManager<S, T> {
    fn new(list: RangeSpacedList<S, T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: RangeLocks::default(),
        }))
    }

    fn indices_lock(this: Rc<RefCell<Self>>) -> RangeIndicesLock<S, T> {
        RangeIndicesLock::new(this)
    }

    fn positions_lock(this: Rc<RefCell<Self>>) -> RangePositionsLock<S, T> {
        RangePositionsLock::new(this)
    }

    fn insertions_lock(this: Rc<RefCell<Self>>) -> RangeInsertionsLock<S, T> {
        RangeInsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> RangeDeletionsLock<S, T> {
        RangeDeletionsLock::new(this)
    }*/

    fn values_lock(this: Rc<RefCell<Self>>) -> RangeValuesLock<S, T> {
        RangeValuesLock::new(this)
    }

    fn indices_handle(this: Rc<RefCell<Self>>) -> RangeIndicesHandle<S, T> {
        RangeIndicesHandle::new(this)
    }

    fn positions_handle(this: Rc<RefCell<Self>>) -> RangePositionsHandle<S, T> {
        RangePositionsHandle::new(this)
    }

    fn insertions_handle(this: Rc<RefCell<Self>>) -> RangeInsertionsHandle<S, T> {
        RangeInsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> RangeDeletionsHandle<S, T> {
        RangeDeletionsHandle::new(this)
    }*/

    fn values_handle(this: Rc<RefCell<Self>>) -> RangeValuesHandle<S, T> {
        RangeValuesHandle::new(this)
    }
}
