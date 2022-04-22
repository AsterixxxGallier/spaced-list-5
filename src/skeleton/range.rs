use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::skeleton::{Range, Skeleton, Spacing};
use crate::skeleton::position::BoundType;

impl<S: Spacing, T> Skeleton<Range, S, T> {
    pub(crate) fn push(&mut self, distance: S, span: S, element: T) {
        if self.elements.is_empty() {
            self.offset = distance;
            self.push_link();
            self.inflate(0, span);
            self.elements.push(element);
            return;
        }
        let index = self.push_link();
        self.inflate(index, distance);
        let index = self.push_link();
        self.inflate(index, span);
        self.elements.push(element);
    }

    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, span: S, element: T) {
        if this.borrow().elements.is_empty() {
            return this.borrow_mut().push(position, span, element);
        }
        if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_span = this.borrow().links[0];
            let previous_first_element =
                mem::replace(&mut this.borrow_mut().elements[0], element);
            this.borrow_mut().inflate_after_offset(previous_first_position - position);
            this.borrow_mut().offset = position;
            this.borrow_mut().links[0] = span;
            return Self::insert(
                this,
                previous_first_position,
                previous_first_span,
                previous_first_element,
            );
        }
        if position >= this.borrow().last_position() {
            return this.borrow_mut().push(
                position - this.borrow().last_position(),
                span,
                element,
            );
        }
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        assert_eq!(BoundType::of(result.index), BoundType::End,
                   "Cannot insert range inside of another range");
        let space_between = this.borrow().link(result.index);
        let sub = Self::ensure_sub(this, result.index);
        assert!(position - result.position + span < space_between,
                "Cannot insert range that intersects another range");
        return Self::insert(sub, position - result.position, span, element);
    }
}