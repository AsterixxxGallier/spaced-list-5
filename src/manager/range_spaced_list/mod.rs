use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

use crate::{Range, Position, Spacing, RangeSpacedList};
use crate::manager::{RangeInsertionsHandle, RangePositionsHandle, RangeValuesHandle,
                     RangeInsertionsLock, RangePositionsLock, RangeValuesLock};

pub mod locks;
pub mod handles;

pub struct RangeLockedPosition<S: Spacing, T> {
    position: Position<Range, S, T>,
    lock: RangePositionsLock<S, T>,
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
    #[must_use]
     pub fn new(list: RangeSpacedList<S, T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            list,
            locks: RangeLocks::default(),
        }))
    }

    pub(crate) fn lock(this: Rc<RefCell<Self>>, position: Position<Range, S, T>) -> RangeLockedPosition<S, T> {
        RangeLockedPosition {
            position,
            lock: RangeManager::positions_lock(this),
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


    pub fn starting_or_ending_before(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_or_ending_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_or_ending_at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_at(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_or_ending_at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_or_ending_at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_or_ending_after(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_or_ending_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }


    pub fn starting_before(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_at(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn starting_after(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.starting_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }


    pub fn ending_before(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.ending_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_at_or_before(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.ending_at_or_before(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_at(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.ending_at(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_at_or_after(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.ending_at_or_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn ending_after(this: Rc<RefCell<Self>>, position: S) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.ending_after(position)
            .map(|position| Self::lock(this.clone(), position))
    }


    pub fn conditional_starting_or_ending_before(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_or_ending_before(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_or_ending_at_or_before(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_or_ending_at_or_before(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_or_ending_at(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_or_ending_at(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_or_ending_at_or_after(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_or_ending_at_or_after(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_or_ending_after(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_or_ending_after(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }


    pub fn conditional_starting_before(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_before(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_at_or_before(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_at_or_before(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_at(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_at(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_at_or_after(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_at_or_after(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_starting_after(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_starting_after(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }


    pub fn conditional_ending_before(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_ending_before(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_ending_at_or_before(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_ending_at_or_before(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_ending_at(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_ending_at(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_ending_at_or_after(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_ending_at_or_after(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }

    pub fn conditional_ending_after(this: Rc<RefCell<Self>>, position: S, condition: fn(Ref<T>) -> bool) -> Option<RangeLockedPosition<S, T>> {
        this.borrow().list.conditional_ending_after(position, condition)
            .map(|position| Self::lock(this.clone(), position))
    }
}
