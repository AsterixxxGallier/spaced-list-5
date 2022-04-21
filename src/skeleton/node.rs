use std::cell::{RefCell};
use std::mem;
use std::rc::Rc;

use crate::skeleton::{Node, Skeleton, Spacing};

impl<S: Spacing, T> Skeleton<Node, S, T> {
    pub(crate) fn push(&mut self, distance: S, element: T) {
        if self.elements.is_empty() {
            self.offset = distance;
            self.elements.push(element);
            return;
        }
        let index = self.push_link();
        self.inflate(index, distance);
        self.elements.push(element);
    }

    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, element: T) {
        if this.borrow().elements.is_empty() {
            return this.borrow_mut().push(position, element);
        }
        if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_element =
                mem::replace(&mut this.borrow_mut().elements[0], element);
            this.borrow_mut().inflate_after_offset(previous_first_position - position);
            this.borrow_mut().offset = position;
            return Self::insert(
                this,
                previous_first_position,
                previous_first_element
            );
        }
        if position >= this.borrow().last_position() {
            return this.borrow_mut().push(
                position - this.borrow().last_position(),
                element
            );
        }
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        let sub = Self::ensure_sub(this, result.index());
        return Self::insert(sub, position - result.position(), element);
    }
}