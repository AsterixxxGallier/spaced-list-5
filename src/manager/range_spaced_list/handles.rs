use std::cell::RefCell;
use std::rc::Rc;

use crate::{Spacing, RangePushError, RangeInsertionError, SpacingError};
use crate::manager::{RangeManager, RangeLockedPosition};

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
// handle!(RangeDeletionsHandle, deletions);
handle!(RangeValuesHandle, values);

impl<S: Spacing, T> RangePositionsHandle<S, T> {
    pub fn try_increase_spacing_after(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
        self.manager.borrow_mut().list.try_increase_spacing_after(position, spacing)
    }

    pub fn try_increase_spacing_before(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
        self.manager.borrow_mut().list.try_increase_spacing_before(position, spacing)
    }

    pub fn try_decrease_spacing_after(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
        self.manager.borrow_mut().list.try_decrease_spacing_after(position, spacing)
    }

    pub fn try_decrease_spacing_before(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
        self.manager.borrow_mut().list.try_decrease_spacing_before(position, spacing)
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
