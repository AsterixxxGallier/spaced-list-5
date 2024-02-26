use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use num_traits::zero;
use thiserror::Error;

use crate::{ElementSlot, EphemeralIndex, EphemeralPosition, Index, Node, Skeleton, Spacing};

#[derive(Error, Debug)]
pub enum PushError {
    // TODO replace distance terminology with "spacing" terminology of the public API
    #[error("Cannot push at a negative distance from the end of a non-empty list.")]
    NegativeDistanceInNonEmptyList,
}

impl<S: Spacing, T> Skeleton<Node, S, T> {
    pub(crate) fn try_push(this: Rc<RefCell<Self>>, distance: S, element: T)
                           -> Result<EphemeralPosition<Node, S, T>, PushError> {
        if this.borrow().elements.is_empty() {
            this.borrow_mut().offset = distance;
            this.borrow_mut().elements.push(ElementSlot::Some(element));
            Ok(EphemeralPosition::new(this, 0, distance))
        } else if distance < zero() {
            Err(PushError::NegativeDistanceInNonEmptyList)
        } else {
            // last element slot must always be full => no need to handle the case that it's empty

            let index = this.borrow_mut().push_link();
            // cannot fail because we would have returned with an Err already if distance were < 0
            this.borrow_mut().increase_spacing(index, distance);
            this.borrow_mut().elements.push(ElementSlot::Some(element));
            Ok(EphemeralPosition::at_end(this))
        }
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
    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, element: T)
                         -> EphemeralPosition<Node, S, T> {
        // TODO rewrite this recursion as iteration, as the recursion can lead to stack overflows
        //  (for example when inserting at a million random u32s into a skeleton)

        if this.borrow().elements.is_empty() {
            // cannot fail, because try_push can only fail when the list is non-empty, which it can't be in this branch
            Self::try_push(this, position, element).unwrap()
        } else if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_element_slot =
                mem::replace(&mut this.borrow_mut().elements[0], ElementSlot::Some(element));

            // cannot fail, because we already established previous_first_position > position
            this.borrow_mut().increase_spacing_after_index(0, previous_first_position - position);
            this.borrow_mut().offset = position;

            if let ElementSlot::Some(previous_first_element) = previous_first_element_slot {
                // reinsert old element

                let insertion_index = Self::insert(
                    this.clone(),
                    previous_first_position,
                    previous_first_element,
                ).into_index();

                let first_persistent_index = this.borrow().first_persistent_index;
                this.borrow_mut().from_persistent.insert(first_persistent_index, insertion_index);
            }

            this.borrow_mut().first_persistent_index -= 1;
            let first_persistent_index = this.borrow().first_persistent_index;

            let first_index = EphemeralIndex::new(this.clone(), 0);
            this.borrow_mut().from_persistent.insert(first_persistent_index, first_index);

            let first_index = Index::new(this.clone(), first_persistent_index);
            this.borrow_mut().into_persistent.insert(0, first_index);

            EphemeralPosition::new(this, 0, position)
        } else if position >= this.borrow().last_position() {
            let distance = position - this.borrow().last_position();
            // cannot fail, because distance cannot be non-negative (by definition and the condition of this branch)
            return Self::try_push(this, distance, element).unwrap();
        } else {
            // TODO for (nested) range too

            // TODO make this more efficient (and not use two queries)
            // cannot be None (medium confidence)
            let at_or_before = Self::shallow_at_or_before(this.clone(), position).unwrap();
            let at_or_after = Self::shallow_at_or_after(this.clone(), position).unwrap();

            // TODO this if statement might be redundant / can probs be simplified
            if at_or_before.index == at_or_after.index && at_or_before.element().is_none() {
                // great, let's put our element there
                *at_or_before.element_mut() = ElementSlot::Some(element);
                // no need to adjust spacing, everything's nicely positioned already (what a coincidence!)
                return at_or_before;
            }

            // there is no slot at position

            if let Some(sub) = this.borrow().sub(at_or_before.index)
                .take_if(|sub| position <= at_or_before.position + sub.borrow().last_position()) {
                // we *need* to insert element into this sub
                return EphemeralPosition {
                    position,
                    ..Self::insert(sub, position - at_or_before.position, element)
                };
            }

            // we can safely decrease the spacing between at_or_before and at_or_after so much that
            // we can put our element at the slot at at_or_after if it's empty

            if at_or_after.element().is_none() {
                // great, let's put our element there
                *at_or_after.element_mut() = ElementSlot::Some(element);
                // adjust spacings
                // basically decrease_spacing_before_index(at_or_after.index, ...)
                // can't fail because at_or_after.position is, well, at or after position
                this.borrow_mut().decrease_spacing(at_or_after.index - 1, at_or_after.position - position);
                this.borrow_mut().increase_spacing_after_index(at_or_after.index, position - at_or_after.position);
                return EphemeralPosition::new(this, at_or_after.index, position);
            }

            // if it isn't tho, we can check if maybe we can put it in the at_or_before slot, but
            // only if it doesn't have a sub (because we otherwise couldn't adjust the spacing well)

            if this.borrow().sub(at_or_before.index).is_none() && at_or_before.element().is_none() {
                // great, let's put our element there
                *at_or_before.element_mut() = ElementSlot::Some(element);
                // adjust spacings
                // basically increase_spacing_before_index(at_or_before.index, ...)
                // can't fail because at_or_before.position is, well, at or before position
                this.borrow_mut().increase_spacing(at_or_before.index - 1, at_or_before.position - position);
                this.borrow_mut().decrease_spacing_after_index(at_or_before.index, position - at_or_before.position);
                return EphemeralPosition::new(this, at_or_before.index, position);
            }

            // we couldn't put our element in at_or_before or at_or_after, nor into the existing
            // range of at_or_before's sub, so we'll just put it in between

            let sub = Self::ensure_sub(this, at_or_before.index);
            EphemeralPosition {
                position,
                ..Self::insert(sub, position - at_or_before.position, element)
            }
        }
    }

    pub(crate) fn remove(this: Rc<RefCell<Self>>, index: EphemeralIndex<Node, S, T>) -> Option<T> {
        // TODO check that EphemeralIndex belongs to this skeleton / take a usize index instead?

        let old_persistent_index = index.persistent();
        this.borrow_mut().dangling_persistent_indices.insert(old_persistent_index.index);
        this.borrow_mut().from_persistent.remove(&old_persistent_index.index);

        this.borrow_mut().first_persistent_index -= 1;
        let new_persistent_index = this.borrow().first_persistent_index;
        this.borrow_mut().into_persistent.insert(index.index, Index::new(this.clone(), new_persistent_index));

        this.borrow_mut().elements[index.index].take()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use itertools::Itertools;
    use rand::random;
    use crate::{Node, Spacing, Skeleton, SpacedList};

    #[test]
    fn removal() {
        let mut list: SpacedList<u32, char> = SpacedList::new();
        let a = list.insert(1, 'a');
        let b = list.insert(2, 'b');
        let c = list.insert(3, 'c');

        list.remove(b.index());

        let mut iter = list.iter();
        assert_eq!('a', iter.next().unwrap().element().unwrap());
        assert_eq!('c', iter.next().unwrap().element().unwrap());
        assert!(iter.next().is_none());
        // assert_eq!(vec!['a', 'c'], list.iter().map(|pos| pos.element().unwrap()).collect_vec());
    }

    #[test]
    fn test() {
        let mut list: SpacedList<i32, char> = SpacedList::new();
        let b = list.insert(0, 'b');
        let c = list.insert(1, 'c');
        let a = list.insert(-1, 'a');

        for pos in [&a, &b, &c] {
            println!("{} at {}; {} in {:?}", pos.element().unwrap(), pos.position, pos.index, pos.skeleton.borrow().elements);
        }

        println!();

        for pos in [&a, &b, &c] {
            println!("{} at {}; {} in {:?}", pos.ephemeral().element().unwrap(), pos.ephemeral().position, pos.ephemeral().index, pos.ephemeral().skeleton.borrow().elements);
        }

        // println!();

        // println!("{} at {} in {:?}", a.ephemeral().element(), a.ephemeral().index, a.ephemeral().skeleton.borrow().elements);
    }

    #[test]
    fn random_structure() {
        let skelly = Skeleton::<Node, u32, ()>::new(None);

        /*
          10 - 3
         100 - 11
        1000 - 65 60 29 36
       10000 - 112 116 198
      100000 - 371 301 ov ov ov
         */

        for _ in 0..1_000 {
            Skeleton::insert(skelly.clone(), random(), ());
        }
        // println!("{}", i);
        // println!("{}", skelly.borrow().elements.len());
        // fn avg_depth<Kind, S: Spacing, T>(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> f64 {
        //     let subs = skeleton.borrow().subs.iter().flatten();
        //     if subs.i
        //     1.0 + subs.map(|sub| avg_depth(sub.clone())).sum::<f64>()
        // }
        fn max_depth<Kind, S: Spacing, T>(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> u64 {
            1 + skeleton.borrow().subs.iter().flatten().map(|sub| max_depth(sub.clone())).max().unwrap_or(0)
        }
        println!("{}", max_depth(skelly));
    }
}