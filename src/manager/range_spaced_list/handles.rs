use std::cell::RefCell;
use std::rc::Rc;
use super::RangeManager;
use crate::{Position, Range, Spacing};

macro_rules! handle {
    ($name:ident, $lock_name:ident) => {
        pub struct $name<S: Spacing, T> {
            manager: Rc<RefCell<RangeManager<S, T>>>
        }

        impl<S: Spacing, T> $name<S, T> {
            pub fn new(manager: Rc<RefCell<RangeManager<S, T>>>) -> Self {
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

handle!(RangeIndicesHandle, indices);
handle!(RangePositionsHandle, positions);
handle!(RangeInsertionsHandle, insertions);
// handle!(RangeDeletionsHandle, deletions);
handle!(RangeValuesHandle, values);

impl<S: Spacing, T> RangePositionsHandle<S, T> {
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

impl<S: Spacing, T> RangeInsertionsHandle<S, T> {
    pub fn push(&self, spacing: S, span: S, value: T) -> Position<Range, S, T> {
        self.manager.borrow_mut().list.push(spacing, span, value)
    }

    pub fn insert_after_start(&self, start: S, end: S, value: T) -> Position<Range, S, T> {
        assert!(start >= self.manager.borrow().list.start());
        self.manager.borrow_mut().list.insert(start, end, value)
    }

    pub fn insert(&self, start: S, end: S, value: T, _indices_handle: &RangeIndicesHandle<S, T>)
                  -> Position<Range, S, T> {
        self.manager.borrow_mut().list.insert(start, end, value)
    }

    pub fn insert_with_span_after_start(&self, start: S, span: S, value: T) -> Position<Range, S, T> {
        assert!(start >= self.manager.borrow().list.start());
        self.manager.borrow_mut().list.insert_with_span(start, span, value)
    }

    pub fn insert_with_span(&self, start: S, span: S, value: T, _indices_handle: &RangeIndicesHandle<S, T>)
                  -> Position<Range, S, T> {
        self.manager.borrow_mut().list.insert_with_span(start, span, value)
    }
}
