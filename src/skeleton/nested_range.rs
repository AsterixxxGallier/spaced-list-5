use std::cell::RefCell;
use std::cmp::Ordering;
use std::mem;
use std::rc::Rc;

use crate::EphemeralPosition;
use crate::skeleton::{NestedRange, Skeleton, Spacing};
use crate::skeleton::index::{EphemeralIndex, Index};
use crate::skeleton::position::BoundType;

#[derive(Debug)]
pub enum NestedRangePushError {
    /// "Cannot push range at a negative distance from the end of a non-empty list"
    NegativeDistanceInNonEmptyList,
    /// "Cannot push range with negative span"
    NegativeSpan,
}

#[derive(Debug)]
pub enum NestedRangeInsertionError {
    /// "Inner range exceeds span of outer range"
    InnerRangeExceedsOuterRange,
    /// "Cannot insert range that intersects another range"
    RangeIntersectsExistingRange,
    /// "Cannot insert range with negative span"
    NegativeSpan,
}

impl<S: Spacing, T> Skeleton<NestedRange, S, T> {
    pub(crate) fn try_push(this: Rc<RefCell<Self>>, distance: S, span: S, element: T)
                           -> Result<EphemeralPosition<NestedRange, S, T>, NestedRangePushError> {
        if span < S::zero() {
            Err(NestedRangePushError::NegativeSpan)
        } else if this.borrow_mut().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().push_link();
            // cannot fail because we would have returned with an Err already if span were < 0
            this.borrow_mut().try_inflate(0, span).unwrap();
            this.borrow_mut().elements.push(element);
            Ok(EphemeralPosition::new(this, 0, distance))
        } else if distance < S::zero() {
            Err(NestedRangePushError::NegativeDistanceInNonEmptyList)
        } else {
            let start_index = this.borrow_mut().push_link();
            // cannot fail because we would have returned with an Err already if distance were < 0
            this.borrow_mut().try_inflate(start_index, distance).unwrap();
            let start_position = this.borrow().last_position();
            let span_index = this.borrow_mut().push_link();
            // cannot fail because we would have returned with an Err already if span were < 0
            this.borrow_mut().try_inflate(span_index, span).unwrap();
            this.borrow_mut().elements.push(element);
            Ok(EphemeralPosition::new(this, span_index, start_position))
        }
    }

    pub(crate) fn try_insert(this: Rc<RefCell<Self>>, position: S, span: S, element: T)
                             -> Result<EphemeralPosition<NestedRange, S, T>, NestedRangeInsertionError> {
        if span < S::zero() {
            Err(NestedRangeInsertionError::NegativeSpan)
        } else if this.borrow().elements.is_empty() {
            // we checked that span is non-negative, so NegativeSpan can't occur, and
            // we checked that the list is empty, so NegativeDistanceInNonEmptyList can't occur
            // so this cannot fail
            Ok(Self::try_push(this, position, span, element).unwrap())
        } else if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_span = this.borrow().links[0];

            if position + span > previous_first_position {
                return Err(NestedRangeInsertionError::RangeIntersectsExistingRange);
            }

            let previous_first_element =
                mem::replace(&mut this.borrow_mut().elements[0], element);

            this.borrow_mut().offset = position;
            match span.cmp(&previous_first_span) {
                Ordering::Greater => {
                    // cannot fail, because we just established span > previous_first_span
                    this.borrow_mut().try_inflate_after_index(0, span - previous_first_span).unwrap();
                }
                Ordering::Less => {
                    // cannot fail, because we just established span < previous_first_span
                    this.borrow_mut().try_deflate_after_index(0, previous_first_span - span).unwrap();
                }
                Ordering::Equal => {
                    // no change needed
                }
            }
            /*
            premises:
            (1) span >= 0
            (2) previous_first_span >= 0
            (3) position < previous_first_position
            (4) previous_first_position >= position + span

            want to prove:
            (5) previous_first_position + previous_first_span >= position + span

            adding a non-negative number onto a number can't make it any smaller
            previous_first_span is non-negative

            (6) previous_first_position + previous_first_span >= previous_first_position

            via transitivity of >=, (5) follows from (6) and (4)

            therefore, this cannot fail
             */
            this.borrow_mut().try_inflate_after_index(1, (previous_first_position + previous_first_span) - (position + span)).unwrap();

            // cannot fail, because we made enough space
            let insertion_index = Self::try_insert(
                this.clone(),
                previous_first_position,
                previous_first_span,
                previous_first_element,
            ).unwrap().into_index();

            let first_persistent_index = this.borrow().first_persistent_index;
            this.borrow_mut().from_persistent.insert(first_persistent_index, insertion_index.clone());
            this.borrow_mut().from_persistent.insert(first_persistent_index + 1, insertion_index.into_range().1);

            this.borrow_mut().first_persistent_index -= 2;
            let first_persistent_index = this.borrow().first_persistent_index;

            let first_index = EphemeralIndex::new(this.clone(), 0);
            this.borrow_mut().from_persistent.insert(first_persistent_index, first_index.clone());
            this.borrow_mut().from_persistent.insert(first_persistent_index + 1, first_index.into_range().1);

            let first_index = Index::new(this.clone(), first_persistent_index);
            this.borrow_mut().into_persistent.insert(0, first_index.clone());
            this.borrow_mut().into_persistent.insert(1, first_index.into_range().1);

            return Ok(EphemeralPosition::new(this, 0, position));
        } else if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            // we checked that span is non-negative, so NegativeSpan can't occur, and
            // distance cannot be negative either (see its definition and the line above)
            // so this cannot fail
            return Ok(Self::try_push(this, distance, span, element).unwrap());
        } else {
            let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
            match BoundType::of(result.index.try_into().unwrap()) {
                BoundType::Start => {
                    let outer_span = result.span();
                    let sub = Self::ensure_sub(this, result.index);
                    if position + span > result.position + outer_span {
                        return Err(NestedRangeInsertionError::InnerRangeExceedsOuterRange)
                    }
                    Ok(EphemeralPosition {
                        position,
                        ..Self::try_insert(sub, position - result.position, span, element)?
                    })
                }
                BoundType::End => {
                    let space_between = this.borrow().link(result.index);
                    let sub = Self::ensure_sub(this, result.index);
                    // I have some doubts if this should be > or >=
                    if position + span > result.position + space_between {
                        return Err(NestedRangeInsertionError::RangeIntersectsExistingRange)
                    }
                    Ok(EphemeralPosition {
                        position,
                        ..Self::try_insert(sub, position - result.position, span, element)?
                    })
                }
            }
        }
    }
}