use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{HollowRangeSpacedList, Spacing};
use self::handles::{HollowRangeIndicesHandle, HollowRangeInsertionsHandle, HollowRangePositionsHandle, HollowRangeValuesHandle};
use self::locks::{HollowRangeIndicesLock, HollowRangeInsertionsLock, HollowRangePositionsLock, HollowRangeValuesLock};

pub mod locks;
pub mod handles;

#[derive(Default)]
struct HollowRangeLocks {
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

pub struct HollowRangeManager<S: Spacing> {
    list: HollowRangeSpacedList<S>,
    locks: HollowRangeLocks,
}

impl<S: Spacing> HollowRangeManager<S> {
    fn new(list: HollowRangeSpacedList<S>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: HollowRangeLocks::default(),
        }))
    }

    fn indices_lock(this: Rc<RefCell<Self>>) -> HollowRangeIndicesLock<S> {
        HollowRangeIndicesLock::new(this)
    }

    fn positions_lock(this: Rc<RefCell<Self>>) -> HollowRangePositionsLock<S> {
        HollowRangePositionsLock::new(this)
    }

    fn insertions_lock(this: Rc<RefCell<Self>>) -> HollowRangeInsertionsLock<S> {
        HollowRangeInsertionsLock::new(this)
    }

    /*fn deletions_lock(this: Rc<RefCell<Self>>) -> HollowRangeDeletionsLock<S> {
        HollowRangeDeletionsLock::new(this)
    }*/

    fn indices_handle(this: Rc<RefCell<Self>>) -> HollowRangeIndicesHandle<S> {
        HollowRangeIndicesHandle::new(this)
    }

    fn positions_handle(this: Rc<RefCell<Self>>) -> HollowRangePositionsHandle<S> {
        HollowRangePositionsHandle::new(this)
    }

    fn insertions_handle(this: Rc<RefCell<Self>>) -> HollowRangeInsertionsHandle<S> {
        HollowRangeInsertionsHandle::new(this)
    }

    /*fn deletions_handle(this: Rc<RefCell<Self>>) -> HollowRangeDeletionsHandle<S> {
        HollowRangeDeletionsHandle::new(this)
    }*/
}
