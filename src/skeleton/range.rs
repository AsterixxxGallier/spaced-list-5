use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::EphemeralPosition;
use crate::skeleton::{ClosedRange, OpenNestedRange, Skeleton, Spacing};
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

impl<S: Spacing, T> Skeleton<OpenNestedRange, S, T> {
    pub(crate) fn push(this: Rc<RefCell<Self>>, distance: S, bound_type: BoundType, element: T) -> EphemeralPosition<OpenNestedRange, S, T> {
        if this.borrow_mut().elements.is_empty() {
            assert_eq!(bound_type, BoundType::Start,
                "Lists must start with a starting range bound");
            this.borrow_mut().offset = distance;
            this.borrow_mut().elements.push(element);
            return EphemeralPosition::new(this, 0, distance);
        }
        // the index of the start of the pushed bound, if it were to be pushed in this skeleton
        let index = this.borrow().elements.len();
        if BoundType::of(index.try_into().unwrap()) == bound_type {
            let index = this.borrow_mut().push_link();
            this.borrow_mut().inflate(index, distance);
            this.borrow_mut().elements.push(element);
            let position = this.borrow().last_position();
            EphemeralPosition::new(this, index, position)
        } else if bound_type == BoundType::Start {
            let sub = Self::ensure_sub(this, index);
            Self::push(sub, distance, bound_type, element)
        } else {
            panic!("Cannot push ending bound to list that does not have a starting bound at its end")
        }
    }

    pub(crate) fn push_range(this: Rc<RefCell<Self>>, distance: S, span: S, element: T) -> EphemeralPosition<OpenNestedRange, S, T> {
        if this.borrow_mut().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().push_link();
            this.borrow_mut().inflate(0, span);
            this.borrow_mut().elements.push(element);
            return EphemeralPosition::new(this, 0, distance);
        }
        // the index of the start of the pushed range, if it were to be pushed in this skeleton
        let index = this.borrow().elements.len();
        match BoundType::of(index.try_into().unwrap()) {
            BoundType::Start => {
                let start_index = this.borrow_mut().push_link();
                this.borrow_mut().inflate(start_index, distance);
                let start_position = this.borrow().last_position();
                let span_index = this.borrow_mut().push_link();
                this.borrow_mut().inflate(span_index, span);
                this.borrow_mut().elements.push(element);
                EphemeralPosition::new(this, start_index, start_position)
            }
            BoundType::End => {
                let sub = Self::ensure_sub(this, index);
                Self::push_range(sub, distance, span, element)
            }
        }
    }

    pub(crate) fn insert_range(this: Rc<RefCell<Self>>, position: S, span: S, element: T) -> EphemeralPosition<OpenNestedRange, S, T> {
        if this.borrow().elements.is_empty() {
            return Self::push_range(this, position, span, element);
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
            return Self::insert_range(
                this,
                previous_first_position,
                previous_first_span,
                previous_first_element,
            );
        }
        if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            return Self::push_range(this, distance, span, element);
        }
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        match BoundType::of(result.index.try_into().unwrap()) {
            BoundType::Start => {
                let outer_span = result.span().unwrap();
                let sub = Self::ensure_sub(this, result.index);
                assert!(position - result.position + span < outer_span,
                        "Inner range exceeds span of outer range");
                EphemeralPosition {
                    position,
                    ..Self::insert_range(sub, position - result.position, span, element)
                }
            }
            BoundType::End => {
                let space_between = this.borrow().link(result.index);
                let sub = Self::ensure_sub(this, result.index);
                assert!(position - result.position + span < space_between,
                        "Cannot insert range that intersects another range");
                EphemeralPosition {
                    position,
                    ..Self::insert_range(sub, position - result.position, span, element)
                }
            }
        }
    }
}