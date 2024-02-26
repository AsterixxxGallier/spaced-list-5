use std::cell::RefCell;
use std::rc::Rc;

use crate::{RangeInsertionError, RangePushError, SpacingError, Spacing};
use crate::manager::{RangeLockedPosition, RangeManager};

macro_rules! handle {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<S: Spacing, T> {
            manager: Rc<RefCell<RangeManager<S, T>>>
        }

        impl<S: Spacing, T> $name<S, T> {
            pub fn new(manager: Rc<RefCell<RangeManager<S, T>>>) -> Self {
                assert_eq!(manager.borrow().locks.$lock_name.get(), 0);
                manager.borrow().locks.$lock_name.set(-1);
                Self {
                    manager
                }
            }
        }

        impl<S: Spacing, T> Drop for $name<S, T> {
            fn drop(&mut self) {
                assert_eq!(self.manager.borrow().locks.$lock_name.get(), -1);
                self.manager.borrow().locks.$lock_name.set(0);
            }
        }
    };
}

handle!(RangePositionsHandle, positions);
handle!(RangeInsertionsHandle, insertions);
// handle!(RangeRemovalsHandle, removals);
handle!(RangeValuesHandle, values);

impl<S: Spacing, T> RangePositionsHandle<S, T> {
    pub fn increase_spacing_after(&mut self, position: S, change: S) {
        self.manager.borrow_mut().list.increase_spacing_after(position, change);
    }

    pub fn increase_spacing_before(&mut self, position: S, change: S) {
        self.manager.borrow_mut().list.increase_spacing_before(position, change);
    }

    pub fn decrease_spacing_after(&mut self, position: S, change: S) {
        self.manager.borrow_mut().list.decrease_spacing_after(position, change);
    }

    pub fn decrease_spacing_before(&mut self, position: S, change: S) {
        self.manager.borrow_mut().list.decrease_spacing_before(position, change);
    }


    pub fn try_increase_spacing_after(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
        self.manager.borrow_mut().list.try_increase_spacing_after(position, change)
    }

    pub fn try_increase_spacing_before(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
        self.manager.borrow_mut().list.try_increase_spacing_before(position, change)
    }

    pub fn try_decrease_spacing_after(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
        self.manager.borrow_mut().list.try_decrease_spacing_after(position, change)
    }

    pub fn try_decrease_spacing_before(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
        self.manager.borrow_mut().list.try_decrease_spacing_before(position, change)
    }
}

impl<S: Spacing, T> RangeInsertionsHandle<S, T> {
    pub fn try_push(&self, spacing: S, span: S, value: T) -> Result<RangeLockedPosition<S, T>, RangePushError> {
        Ok(RangeManager::lock(self.manager.clone(), self.manager.borrow_mut().list.try_push(spacing, span, value)?))
    }

    pub fn try_insert(&self, start: S, end: S, value: T) -> Result<RangeLockedPosition<S, T>, RangeInsertionError> {
        Ok(RangeManager::lock(self.manager.clone(), self.manager.borrow_mut().list.try_insert(start, end, value)?))
    }

    pub fn try_insert_with_span(&self, start: S, span: S, value: T) -> Result<RangeLockedPosition<S, T>, RangeInsertionError> {
        Ok(RangeManager::lock(self.manager.clone(), self.manager.borrow_mut().list.try_insert_with_span(start, span, value)?))
    }
}
