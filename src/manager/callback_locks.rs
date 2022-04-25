use std::cell::RefCell;
use std::rc::Rc;

use crate::manager::callbacks::IndexChange;
use crate::manager::callbacks::SpacingChange;
use crate::manager::callbacks::Insertion;
use crate::manager::Manager;
use crate::Spacing;

macro_rules! callback_lock {
    ($name:ident, $lock_name:ident, $param:ty) => {
        pub struct $name<'manager, S: Spacing, T, F: Fn($param)> {
            manager: Rc<RefCell<Manager<'manager, S, T>>>,
            callback: dyn Fn($param),
            key: usize,
        }

        impl<'manager, S: Spacing, T, F: Fn($param)> $name<'manager, S, T, F> {
            pub fn new(manager: Rc<RefCell<Manager<'manager, S, T>>>, callback: F) -> Self {
                let key = manager.borrow().callbacks.$lock_name
                    .borrow_mut().insert(&callback);
                Self {
                    manager,
                    callback,
                    key,
                }
            }
        }

        impl<'manager, S: Spacing, T, F: Fn($param)> Drop for $name<'manager, S, T, F> {
            fn drop(&mut self) {
                self.manager.borrow().callbacks.$lock_name.borrow_mut().remove(self.key);
            }
        }
    };
}

callback_lock!(IndicesCallbackLock, indices, IndexChange<S, T>);
callback_lock!(PositionsCallbackLock, positions, SpacingChange<S>);
callback_lock!(InsertionsCallbackLock, insertions, Insertion<S, T>);