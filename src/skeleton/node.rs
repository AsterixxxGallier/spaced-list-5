use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::EphemeralPosition;
use crate::skeleton::{Node, Skeleton, Spacing};

impl<S: Spacing, T> Skeleton<Node, S, T> {
    pub(crate) fn push(this: Rc<RefCell<Self>>, distance: S, element: T) -> EphemeralPosition<Node, S, T> {
        if this.borrow().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().elements.push(element);
            // return Position::new(this, 0, distance);
            return EphemeralPosition::new(this, 0, distance);
        }
        let index = this.borrow_mut().push_link();
        this.borrow_mut().inflate(index, distance);
        this.borrow_mut().elements.push(element);
        EphemeralPosition::at_end(this)
    }

    //    ________8________
    //    ____5____
    //    __3__   __2__
    //  4 | 3 | 2 | 2 | 1 |
    //    A   B   C   D   E            A is now at index 0
    //                                 persistent index of A = 0
    // insert at 3:
    // make space
    //    ________9________
    //    ____6____
    //    __4__   __2__
    //  3 | 4 | 2 | 2 | 1 |
    //    A   B   C   D   E
    // insert new element
    //    ________9________
    //    ____6____
    //    __4__   __2__
    //  3 | 4 | 2 | 2 | 1 |
    //    F   B   C   D   E            F is now at index 0 (ike)
    // reinsert old element            persistent index of F = -1
    //    ________9________
    //    ____6____
    //    __4__   __2__
    //  3 | 4 | 2 | 2 | 1 |
    //    F   B   C   D   E            counter = 1
    //    sub:
    //     1 |                         persistent index of B = 1
    //       A                         persistent index of A = 0
    //                                 persistent index of F = -1
    //
    //                                 access at 1 => self at 1
    //                                 access at 0 => get new index with skeleton from map/vec
    //                                 access at -1 => self at 0
    //
    //
    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, element: T) -> EphemeralPosition<Node, S, T> {
        if this.borrow().elements.is_empty() {
            return Self::push(this, position, element);
        }
        if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_element =
                mem::replace(&mut this.borrow_mut().elements[0], element);
            this.borrow_mut().inflate_after_offset(previous_first_position - position);
            this.borrow_mut().offset = position;
            let pos = Self::insert(
                this.clone(),
                previous_first_position,
                previous_first_element,
            );
            // FIXME Problem:
            //        After the offset-swap-around maneuver, the first element will not be at the
            //        persistent index 0, but at a negative one!

            // TODO for range too
            let first_persistent_index = this.borrow().first_persistent_index;
            this.borrow_mut().from_persistent.insert(first_persistent_index, pos);
            this.borrow_mut().first_persistent_index -= 1;
            // return EphemeralPosition::persistent_new(this, -1 /*TODO*/, position);
            return EphemeralPosition::new(this, 0, position);
        }
        if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            return Self::push(this, distance, element);
        }
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        let sub = Self::ensure_sub(this, result.index);
        EphemeralPosition {
            position,
            ..Self::insert(sub, position - result.position, element)
        }
    }
}