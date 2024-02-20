use std::cell::RefCell;
use std::rc::Rc;

use crate::{Spacing, RangePushError, RangeInsertionError, SpacingError};
use crate::manager::{HollowRangeManager, HollowRangeLockedPosition};

macro_rules! handle {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<S: Spacing> {
            manager: Rc<RefCell<HollowRangeManager<S>>>
        }

        impl<S: Spacing> $name<S> {
            pub fn new(manager: Rc<RefCell<HollowRangeManager<S>>>) -> Self {
                assert_eq!(manager.borrow().locks.$lock_name.get(), 0);
                manager.borrow().locks.$lock_name.set(-1);
                Self {
                    manager
                }
            }
        }

        impl<S: Spacing> Drop for $name<S> {
            fn drop(&mut self) {
                assert_eq!(self.manager.borrow().locks.$lock_name.get(), -1);
                self.manager.borrow().locks.$lock_name.set(0);
            }
        }
    };
}

handle!(HollowRangePositionsHandle, positions);
handle!(HollowRangeInsertionsHandle, insertions);
// handle!(HollowRangeDeletionsHandle, deletions);

impl<S: Spacing> HollowRangePositionsHandle<S> {
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

impl<S: Spacing> HollowRangeInsertionsHandle<S> {
    pub fn try_push(&self, spacing: S, span: S) -> Result<HollowRangeLockedPosition<S>, RangePushError> {
        Ok(HollowRangeManager::lock(self.manager.clone(), self.manager.borrow_mut().list.try_push(spacing, span)?))
    }

    pub fn try_insert(&self, start: S, end: S) -> Result<HollowRangeLockedPosition<S>, RangeInsertionError> {
        Ok(HollowRangeManager::lock(self.manager.clone(), self.manager.borrow_mut().list.try_insert(start, end)?))
    }

    pub fn try_insert_with_span(&self, start: S, span: S) -> Result<HollowRangeLockedPosition<S>, RangeInsertionError> {
        Ok(HollowRangeManager::lock(self.manager.clone(), self.manager.borrow_mut().list.try_insert_with_span(start, span)?))
    }
}
