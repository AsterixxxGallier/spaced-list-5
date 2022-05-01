use std::cell::RefCell;
use std::rc::Rc;

use super::callbacks::IndexChange;
use super::callbacks::SpacingChange;
use super::callbacks::Insertion;
use super::Manager;
use crate::Spacing;

macro_rules! callback_lock {
    ($name:ident, $lock_name:ident, $param:ty) => {
        pub struct $name<'manager, S: Spacing, T, F: Fn($param)> {
            manager: Rc<RefCell<Manager<'manager, S, T>>>,
            callback: F,
            key: usize,
        }

        impl<'manager, S: Spacing, T, F: Fn($param)> $name<'manager, S, T, F> {
            pub fn new(manager: Rc<RefCell<Manager<'manager, S, T>>>, callback: F) -> Self {
                let mut this = Self {
                    manager,
                    callback,
                    key: 0,
                };
                let key = this.manager.borrow().callbacks.$lock_name
                    .borrow_mut().insert(&this.callback);
                this.key = key;
                this
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