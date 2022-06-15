use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::EphemeralPosition;
use crate::skeleton::{Node, Skeleton, Spacing};
use crate::skeleton::index::{EphemeralIndex, Index};

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
    // ephemeral 0 => persistent -1
    // persistent -1 => ephemeral 0
    // persistent 0 => ephemeral sub/0
    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, element: T) -> EphemeralPosition<Node, S, T> {
        if this.borrow().elements.is_empty() {
            return Self::push(this, position, element);
        }
        // TODO for range too
        if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_element =
                mem::replace(&mut this.borrow_mut().elements[0], element);

            this.borrow_mut().inflate_after_offset(previous_first_position - position);
            this.borrow_mut().offset = position;

            let insertion_index = Self::insert(
                this.clone(),
                previous_first_position,
                previous_first_element,
            ).into_index();

            let first_persistent_index = this.borrow().first_persistent_index;

            this.borrow_mut().from_persistent.insert(first_persistent_index, insertion_index);

            this.borrow_mut().first_persistent_index -= 1;

            let first_persistent_index = first_persistent_index - 1;

            let first_index =
                EphemeralIndex::new(this.clone(), 0);
            this.borrow_mut().from_persistent.insert(first_persistent_index, first_index);

            let first_index =
                Index::new(this.clone(), first_persistent_index);
            this.borrow_mut().into_persistent.insert(0, first_index);

            // let position = Position::new(this.clone(), first_persistent_index, previous_first_position);
            // this.borrow_mut().into_persistent.insert(-first_persistent_index as usize, position);
            // return EphemeralPosition::persistent_new(this, -1 /*TODO*/, position);
            return EphemeralPosition::new(this, 0, position);
        }
        /*        if position < this.borrow().offset {
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
                    let insertion_position = pos.position;
                    // FIXME Problem:
                    //        After the offset-swap-around maneuver, the first element will not be at the
                    //        persistent index 0, but at a negative one!
        
                    // TODO for range too
                    let first_persistent_index = this.borrow().first_persistent_index;
                    this.borrow_mut().from_persistent.insert(first_persistent_index, pos);
                    this.borrow_mut().first_persistent_index -= 1;
                    let position = Position::new(this.clone(), this.borrow().first_persistent_index, insertion_position);
                    this.borrow_mut().into_persistent.insert(-first_persistent_index as usize, position);
                    // return EphemeralPosition::persistent_new(this, -1 /*TODO*/, position);
                    return EphemeralPosition::new(this, 0, insertion_position);
                }
        */        if position >= this.borrow().last_position() {
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

#[cfg(test)]
mod tests {
    use crate::SpacedList;

    #[test]
    fn test() {
        let mut list: SpacedList<i32, char> = SpacedList::new();
        let b = list.insert(0, 'b');
        let c = list.insert(1, 'c');
        let a = list.insert(-1, 'a');

        println!("{} at {}; {} in {:?}", a.ephemeral().element(), a.position, a.index, a.skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", b.ephemeral().element(), b.position, b.index, b.skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", c.ephemeral().element(), c.position, c.index, c.skeleton.borrow().elements);

        println!();

        println!("{} at {}; {} in {:?}", a.ephemeral().element(), a.ephemeral().position, a.ephemeral().index, a.ephemeral().skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", b.ephemeral().element(), b.ephemeral().position, b.ephemeral().index, b.ephemeral().skeleton.borrow().elements);
        println!("{} at {}; {} in {:?}", c.ephemeral().element(), c.ephemeral().position, c.ephemeral().index, c.ephemeral().skeleton.borrow().elements);

        // println!();

        // println!("{} at {} in {:?}", a.ephemeral().element(), a.ephemeral().index, a.ephemeral().skeleton.borrow().elements);
    }
}