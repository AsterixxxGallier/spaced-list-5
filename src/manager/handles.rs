use std::cell::RefCell;
use std::rc::Rc;
use crate::manager::Manager;
use crate::{Node, Position, Spacing};

macro_rules! handle {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<'manager, S: Spacing, T> {
            manager: Rc<RefCell<Manager<'manager, S, T>>>
        }

        impl<'manager, S: Spacing, T> $name<'manager, S, T> {
            pub fn new(manager: Rc<RefCell<Manager<'manager, S, T>>>) -> Self {
                assert_eq!(manager.borrow().locks.$lock_name.get(), 0);
                manager.borrow().locks.$lock_name.set(-1);
                Self {
                    manager
                }
            }
        }

        impl<'manager, S: Spacing, T> Drop for $name<'manager, S, T> {
            fn drop(&mut self) {
                assert_eq!(self.manager.borrow().locks.$lock_name.get(), -1);
                self.manager.borrow().locks.$lock_name.set(0);
            }
        }
    };
}

handle!(IndicesHandle, indices);
handle!(PositionsHandle, positions);
handle!(InsertionsHandle, insertions);
// handle!(DeletionsHandle, deletions);
handle!(ValuesHandle, values);

impl<'manager, S: Spacing, T> PositionsHandle<'manager, S, T> {
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

impl<'manager, S: Spacing, T> InsertionsHandle<'manager, S, T> {
    pub fn push(&self, spacing: S, value: T) -> Position<Node, S, T> {
        self.manager.borrow_mut().list.push(spacing, value)
    }

    pub fn insert_after_start(&self, position: S, value: T) -> Position<Node, S, T> {
        assert!(position >= self.manager.borrow().list.start());
        self.manager.borrow_mut().list.insert(position, value)
    }

    pub fn insert(&self, position: S, value: T, _indices_handle: &IndicesHandle<S, T>)
                  -> Position<Node, S, T> {
        // TODO call callbacks with OffsetSwapAround when inserting before start
        self.manager.borrow_mut().list.insert(position, value)
    }
}
