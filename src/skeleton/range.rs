use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::EphemeralPosition;
use crate::skeleton::{ClosedRange, Skeleton, Spacing};
use crate::skeleton::position::BoundType;

impl<S: Spacing, T> Skeleton<ClosedRange, S, T> {
    pub(crate) fn push(this: Rc<RefCell<Self>>, distance: S, span: S, element: T) -> EphemeralPosition<ClosedRange, S, T> {
        if this.borrow_mut().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().push_link();
            this.borrow_mut().inflate(0, span);
            this.borrow_mut().elements.push(element);
            return EphemeralPosition::new(this, 0, distance);
        }
        let start_index = this.borrow_mut().push_link();
        this.borrow_mut().inflate(start_index, distance);
        let start_position = this.borrow().last_position();
        let span_index = this.borrow_mut().push_link();
        this.borrow_mut().inflate(span_index, span);
        this.borrow_mut().elements.push(element);
        EphemeralPosition::new(this, start_index, start_position)
    }

    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, span: S, element: T) -> EphemeralPosition<ClosedRange, S, T> {
        if this.borrow().elements.is_empty() {
            return Self::push(this, position, span, element);
        }
        if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_span = this.borrow().links[0];
            let previous_first_element =
                mem::replace(&mut this.borrow_mut().elements[0], element);
            this.borrow_mut().inflate_after_offset(previous_first_position - position);
            this.borrow_mut().offset = position;
            this.borrow_mut().links[0] = span;
            if this.borrow().links.len() == 1 {
                this.borrow_mut().length = span;
            }
            return Self::insert(
                this,
                previous_first_position,
                previous_first_span,
                previous_first_element,
            );
        }
        if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            return Self::push(this, distance, span, element);
        }
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        assert_eq!(BoundType::of(result.index.try_into().unwrap()), BoundType::End,
                   "Cannot insert range inside of another range");
        let space_between = this.borrow().link(result.index);
        let sub = Self::ensure_sub(this, result.index);
        assert!(position - result.position + span < space_between,
                "Cannot insert range that intersects another range");
        EphemeralPosition {
            position,
            ..Self::insert(sub, position - result.position, span, element)
        }
    }
}