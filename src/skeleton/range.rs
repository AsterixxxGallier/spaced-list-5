use std::cell::RefCell;
use std::cmp::Ordering;
use std::mem;
use std::rc::Rc;

use crate::{BoundType, EphemeralIndex, EphemeralPosition, Index, Range, Skeleton, Spacing};

#[derive(Debug)]
pub enum RangePushError {
    /// "Cannot push range at a negative distance from the end of a non-empty list"
    NegativeDistanceInNonEmptyList,
    /// "Cannot push range with negative span"
    NegativeSpan,
}

#[derive(Debug)]
pub enum RangeInsertionError {
    /// "Cannot insert range that starts inside of another range"
    RangeStartsInsideExistingRange,
    /// "Cannot insert range that intersects another range"
    RangeIntersectsExistingRange,
    /// "Cannot insert range with negative span"
    NegativeSpan,
}

impl<S: Spacing, T> Skeleton<Range, S, T> {
    pub(crate) fn try_push(this: Rc<RefCell<Self>>, distance: S, span: S, element: T)
                           -> Result<EphemeralPosition<Range, S, T>, RangePushError> {
        if span < S::zero() {
            Err(RangePushError::NegativeSpan)
        } else if this.borrow_mut().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().push_link();
            // cannot fail because we would have returned with an Err already if span were < 0
            this.borrow_mut().try_inflate(0, span).unwrap();
            this.borrow_mut().elements.push(element);
            Ok(EphemeralPosition::new(this, 0, distance))
        } else if distance < S::zero() {
            Err(RangePushError::NegativeDistanceInNonEmptyList)
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

    //    ____5____
    //    __3__   __2__
    //  4 | 3 | 2 | 2 |
    //      A       B                  A.0 is now at index 0
    //                                 persistent index of A.0 = 0, of A.1 = 1
    //
    // insert C at 1 with span 2:
    // replace old value with new value
    //    ____5____
    //    __3__   __2__
    //  4 | 3 | 2 | 2 |
    //      C       B                  C.0 is now at index 0 (ike)
    // adjust spacings                 persistent index of C.0 = -2, of C.1 = -1
    //    ____8____
    //    __2__   __2__
    //  1 | 2 | 6 | 2 |
    //      C       B
    // adjust spacings (step 1: change offset)
    //    ____5____
    //    __3__   __2__
    //  1 | 3 | 2 | 2 |
    //      C       B
    // adjust spacings (step 2: inflate/deflate after 0, can be both)
    //    ____4____
    //    __2__   __2__
    //  1 | 2 | 2 | 2 |
    //      C       B
    // adjust spacings (step 3: inflate after 1 to restore absolute positions of other elements
    //    ____8____                         and to make space for reinserting the old range)
    //    __2__   __2__
    //  1 | 2 | 6 | 2 |
    //      C       B
    // reinsert old element
    //      C       B
    //    ____8____
    //    __2__   __2__
    //  1 | 2 | 6 | 2 |
    //      C       B
    //        sub:                     persistent indices:
    //           __3__                 A.0 = +0  A.1 = +1
    //         1 | 3 |                 B.0 = +2  B.1 = +3
    //             A                   C.0 = -2  C.1 = -1
    pub(crate) fn try_insert(this: Rc<RefCell<Self>>, position: S, span: S, element: T)
                             -> Result<EphemeralPosition<Range, S, T>, RangeInsertionError> {
        if span < S::zero() {
            Err(RangeInsertionError::NegativeSpan)
        } else if this.borrow().elements.is_empty() {
            // we checked that span is non-negative, so NegativeSpan can't occur, and
            // we checked that the list is empty, so NegativeDistanceInNonEmptyList can't occur
            // so this cannot fail
            Ok(Self::try_push(this, position, span, element).unwrap())
        } else if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_span = this.borrow().links[0];

            if position + span > previous_first_position {
                return Err(RangeInsertionError::RangeIntersectsExistingRange);
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

            Ok(EphemeralPosition::new(this, 0, position))
        } else if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            // we checked that span is non-negative, so NegativeSpan can't occur, and
            // distance cannot be negative either (see its definition and the line above)
            // so this cannot fail
            Ok(Self::try_push(this, distance, span, element).unwrap())
        } else {
            let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
            if BoundType::of(result.index) == BoundType::Start {
                return Err(RangeInsertionError::RangeStartsInsideExistingRange);
            }
            let space_between = this.borrow().link(result.index);
            // I have some doubts if this should be > or >=
            if position + span > result.position + space_between {
                return Err(RangeInsertionError::RangeIntersectsExistingRange);
            }
            let sub = Self::ensure_sub(this, result.index);
            Ok(EphemeralPosition {
                position,
                ..Self::try_insert(sub, position - result.position, span, element)?
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::spaced_lists::range_spaced_list::RangeSpacedList;

    #[test]
    fn test() {
        let mut list: RangeSpacedList<i32, char> = RangeSpacedList::new();
        let b = list.try_insert(0, 2, 'b').unwrap();
        let c = list.try_insert(3, 4, 'c').unwrap();
        let a = list.try_insert(-2, -1, 'a').unwrap();

        println!("{} at {}; {} in {:?}", *a.element(), a.position, a.index, a.skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", *b.element(), b.position, b.index, b.skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", *c.element(), c.position, c.index, c.skeleton.borrow().elements);

        println!();

        println!("{} at {}; {} in {:?}", a.ephemeral().element(), a.ephemeral().position, a.ephemeral().index, a.ephemeral().skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", b.ephemeral().element(), b.ephemeral().position, b.ephemeral().index, b.ephemeral().skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", c.ephemeral().element(), c.ephemeral().position, c.ephemeral().index, c.ephemeral().skeleton.borrow().elements);

        // println!();

        // println!("{} at {} in {:?}", a.ephemeral().element(), a.ephemeral().index, a.ephemeral().skeleton.borrow().elements);
    }
}