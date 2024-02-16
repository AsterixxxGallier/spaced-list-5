use std::cell::RefCell;
use std::rc::Rc;

use crate::Spacing;
use crate::manager::spaced_list::LockedPosition;
use crate::spaced_lists::SpacingError;

use super::Manager;

macro_rules! handle {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<S: Spacing, T> {
            manager: Rc<RefCell<Manager<S, T>>>
        }

        impl<S: Spacing, T> $name<S, T> {
            pub fn new(manager: Rc<RefCell<Manager<S, T>>>) -> Self {
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

handle!(PositionsHandle, positions);
handle!(InsertionsHandle, insertions);
// handle!(DeletionsHandle, deletions);
handle!(ValuesHandle, values);

impl<S: Spacing, T> PositionsHandle<S, T> {
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

impl<S: Spacing, T> InsertionsHandle<S, T> {
    pub fn push(&self, spacing: S, value: T) -> LockedPosition<S, T> {
        Manager::lock(self.manager.clone(), self.manager.borrow_mut().list.push(spacing, value))
    }

    pub fn insert(&self, position: S, value: T) -> LockedPosition<S, T> {
        Manager::lock(self.manager.clone(), self.manager.borrow_mut().list.insert(position, value))
    }
}
