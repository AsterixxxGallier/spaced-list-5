use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::EphemeralPosition;
use crate::skeleton::{Range, Skeleton, Spacing};
use crate::skeleton::position::BoundType;

#[derive(Debug)]
pub enum RangeInsertionError {
    /// "Cannot insert range that starts inside of another range"
    RangeStartsInsideExistingRange,
    /// "Cannot insert range that intersects another range"
    RangeIntersectsExistingRange,
}

impl<S: Spacing, T> Skeleton<Range, S, T> {
    pub(crate) fn push(this: Rc<RefCell<Self>>, distance: S, span: S, element: T)
                       -> EphemeralPosition<Range, S, T> {
        if this.borrow_mut().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().push_link();
            this.borrow_mut().try_inflate(0, span);
            this.borrow_mut().elements.push(element);
            return EphemeralPosition::new(this, 0, distance);
        }
        let start_index = this.borrow_mut().push_link();
        this.borrow_mut().try_inflate(start_index, distance);
        let start_position = this.borrow().last_position();
        let span_index = this.borrow_mut().push_link();
        this.borrow_mut().try_inflate(span_index, span);
        this.borrow_mut().elements.push(element);
        EphemeralPosition::new(this, start_index, start_position)
    }

    pub(crate) fn try_insert(this: Rc<RefCell<Self>>, position: S, span: S, element: T)
                             -> Result<EphemeralPosition<Range, S, T>, RangeInsertionError> {
        if this.borrow().elements.is_empty() {
            return Ok(Self::push(this, position, span, element));
        }
        if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_span = this.borrow().links[0];
            let previous_first_element =
                mem::replace(&mut this.borrow_mut().elements[0], element);
            this.borrow_mut().try_inflate_after_offset(previous_first_position - position);
            this.borrow_mut().offset = position;
            this.borrow_mut().links[0] = span;
            if this.borrow().links.len() == 1 {
                this.borrow_mut().length = span;
            }
            return Self::try_insert(
                this,
                previous_first_position,
                previous_first_span,
                previous_first_element,
            );
        }
        if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            return Ok(Self::push(this, distance, span, element));
        }
        let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
        if BoundType::of(result.index.try_into().unwrap()) == BoundType::Start {
            return Err(RangeInsertionError::RangeStartsInsideExistingRange);
        }
        let space_between = this.borrow().link(result.index);
        // I have some doubts if this should be > or >=
        if position - result.position + span > space_between {
            return Err(RangeInsertionError::RangeIntersectsExistingRange);
        }
        let sub = Self::ensure_sub(this, result.index);
        Ok(EphemeralPosition {
            position,
            ..Self::try_insert(sub, position - result.position, span, element)?
        })
    }
}