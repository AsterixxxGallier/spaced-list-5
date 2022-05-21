use std::cell::RefCell;
use std::rc::Rc;
use super::HollowRangeManager;
use crate::{Range, Spacing};

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
    pub fn increase_spacing_after(&mut self, position: S, spacing: S) {
        self.manager.borrow_mut().list.increase_spacing_after(position, spacing)
    }

    pub fn increase_spacing_before(&mut self, position: S, spacing: S) {
        self.manager.borrow_mut().list.increase_spacing_before(position, spacing)
    }

    pub fn decrease_spacing_after(&mut self, position: S, spacing: S) {
        self.manager.borrow_mut().list.decrease_spacing_after(position, spacing)
    }

    pub fn decrease_spacing_before(&mut self, position: S, spacing: S) {
        self.manager.borrow_mut().list.decrease_spacing_before(position, spacing)
    }
}

impl<S: Spacing> HollowRangeInsertionsHandle<S> {
    pub fn push(&self, spacing: S, span: S) -> HollowPosition<Range, S> {
        self.manager.borrow_mut().list.push(spacing, span)
    }

    pub fn insert(&self, start: S, end: S) -> HollowPosition<Range, S> {
        self.manager.borrow_mut().list.insert(start, end)
    }

    pub fn insert_with_span(&self, start: S, span: S) -> HollowPosition<Range, S> {
        self.manager.borrow_mut().list.insert_with_span(start, span)
    }
}
