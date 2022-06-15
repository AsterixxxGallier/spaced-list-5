use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::{HollowPosition, HollowRangeSpacedList, Range, Spacing};

use self::handles::{HollowRangeInsertionsHandle, HollowRangePositionsHandle};
use self::locks::{HollowRangeInsertionsLock, HollowRangePositionsLock};

pub mod locks;
pub mod handles;

pub struct HollowRangeLockedPosition<S: Spacing> {
    position: HollowPosition<Range, S>,
    lock: HollowRangePositionsLock<S>,
}

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

    pub(crate) fn lock(this: Rc<RefCell<Self>>, position: HollowPosition<Range, S>) -> HollowRangeLockedPosition<S> {
        HollowRangeLockedPosition {
            position,
            lock: HollowRangeManager::positions_lock(this),
        }
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

    pub fn starting_or_ending_before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_or_ending_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_or_ending_at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_at(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_or_ending_at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_or_ending_at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_or_ending_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_at(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.starting_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.ending_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.ending_at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_at(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.ending_at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.ending_at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_after(this: Rc<RefCell<Self>>, position: S) -> Option<HollowRangeLockedPosition<S>> {
        this.borrow().list.ending_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }
}
