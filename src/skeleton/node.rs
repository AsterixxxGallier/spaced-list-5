use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::Position;
use crate::skeleton::{Node, Skeleton, Spacing};

impl<S: Spacing, T> Skeleton<Node, S, T> {
    pub(crate) fn push(this: Rc<RefCell<Self>>, distance: S, element: T) -> Position<Node, S, T> {
        if this.borrow().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().elements.push(element);
            return Position::persistent_new(this, 0, distance);
        }
        let index = this.borrow_mut().push_link();
        this.borrow_mut().inflate(index, distance);
        this.borrow_mut().elements.push(element);
        Position::at_end(this)
    }

    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, element: T) -> Position<Node, S, T> {
        if this.borrow().elements.is_empty() {
            return Self::push(this, position, element);
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
                previous_first_element,
            );
        }
        if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            return Self::push(this, distance, element);
        }
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        let sub = Self::ensure_sub(this, result.index);
        Position {
            position,
            ..Self::insert(sub, position - result.position, element)
        }
    }
}