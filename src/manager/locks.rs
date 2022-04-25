use std::cell::RefCell;
use std::rc::Rc;
use crate::manager::Manager;
use crate::Spacing;

macro_rules! lock {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<'manager, S: Spacing, T> {
            manager: Rc<RefCell<Manager<'manager, S, T>>>
        }

        impl<'manager, S: Spacing, T> $name<'manager, S, T> {
            pub fn new(manager: Rc<RefCell<Manager<'manager, S, T>>>) -> Self {
                assert_ne!(manager.borrow().locks.$lock_name.get(), -1);
                manager.borrow().locks.$lock_name.set(manager.borrow().locks.$lock_name.get() + 1);
                Self {
                    manager
                }
            }
        }

        impl<'manager, S: Spacing, T> Drop for $name<'manager, S, T> {
            fn drop(&mut self) {
                self.manager.borrow().locks.$lock_name.set(
                    self.manager.borrow().locks.$lock_name.get() - 1);
            }
        }
    };
}

lock!(IndicesLock, indices);
lock!(PositionsLock, positions);
lock!(InsertionsLock, insertions);
// lock!(DeletionsLock, deletions);
lock!(ValuesLock, values);