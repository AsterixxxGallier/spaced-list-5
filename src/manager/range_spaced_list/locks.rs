use std::cell::RefCell;
use std::rc::Rc;
use super::RangeManager;
use crate::Spacing;

macro_rules! lock {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<S: Spacing, T> {
            manager: Rc<RefCell<RangeManager<S, T>>>
        }

        impl<S: Spacing, T> $name<S, T> {
            pub fn new(manager: Rc<RefCell<RangeManager<S, T>>>) -> Self {
                assert_ne!(manager.borrow().locks.$lock_name.get(), -1);
                manager.borrow().locks.$lock_name.set(manager.borrow().locks.$lock_name.get() + 1);
                Self {
                    manager
                }
            }
        }

        impl<S: Spacing, T> Drop for $name<S, T> {
            fn drop(&mut self) {
                self.manager.borrow().locks.$lock_name.set(
                    self.manager.borrow().locks.$lock_name.get() - 1);
            }
        }
    };
}

lock!(RangeIndicesLock, indices);
lock!(RangePositionsLock, positions);
lock!(RangeInsertionsLock, insertions);
// lock!(RangeDeletionsLock, deletions);
lock!(RangeValuesLock, values);