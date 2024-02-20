use std::cell::RefCell;
use std::rc::Rc;

use crate::Spacing;
use crate::manager::HollowRangeManager;

macro_rules! lock {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<S: Spacing> {
            manager: Rc<RefCell<HollowRangeManager<S>>>
        }

        impl<S: Spacing> $name<S> {
            pub fn new(manager: Rc<RefCell<HollowRangeManager<S>>>) -> Self {
                assert_ne!(manager.borrow().locks.$lock_name.get(), -1);
                manager.borrow().locks.$lock_name.set(manager.borrow().locks.$lock_name.get() + 1);
                Self {
                    manager
                }
            }
        }

        impl<S: Spacing> Drop for $name<S> {
            fn drop(&mut self) {
                self.manager.borrow().locks.$lock_name.set(
                    self.manager.borrow().locks.$lock_name.get() - 1);
            }
        }
    };
}

lock!(HollowRangePositionsLock, positions);
lock!(HollowRangeInsertionsLock, insertions);
// lock!(HollowRangeDeletionsLock, deletions);