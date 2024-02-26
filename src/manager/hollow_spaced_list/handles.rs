use std::cell::RefCell;
use std::rc::Rc;

use crate::{PushError, SpacingError, Spacing};
use crate::manager::{HollowLockedPosition, HollowManager};

macro_rules! handle {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<S: Spacing> {
            manager: Rc<RefCell<HollowManager<S>>>
        }

        impl<S: Spacing> $name<S> {
            pub fn new(manager: Rc<RefCell<HollowManager<S>>>) -> Self {
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

handle!(HollowPositionsHandle, positions);
handle!(HollowInsertionsHandle, insertions);
// handle!(HollowRemovalsHandle, removals);

impl<S: Spacing> HollowPositionsHandle<S> {
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

impl<S: Spacing> HollowInsertionsHandle<S> {
    pub fn try_push(&self, spacing: S) -> Result<HollowLockedPosition<S>, PushError> {
        Ok(HollowManager::lock(self.manager.clone(), self.manager.borrow_mut().list.try_push(spacing)?))
    }

    pub fn insert(&self, position: S) -> HollowLockedPosition<S> {
        HollowManager::lock(self.manager.clone(), self.manager.borrow_mut().list.insert(position))
    }
}
