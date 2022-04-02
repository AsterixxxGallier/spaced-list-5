use std::marker::PhantomData;
use std::ops::Neg;
use num_traits::zero;

use crate::{SpacedListSkeleton, Spacing, Todo, Position};
use crate::spaced_lists::traversal::node::Traversal;
use crate::spaced_lists::traversal::shallow::{ShallowPosition, ShallowTraversal};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SublistData<S: Spacing, List: SpacedList<S>> {
    containing_list: PhantomData<List>,
    node_index: usize,
    position: S
}

pub trait SpacedList<S: Spacing>: Default {
    fn sublist_data(&self) -> Option<&SublistData<S, Self>>;

    fn skeleton(&self) -> &SpacedListSkeleton<S, Self>;

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self>;

    fn size(&self) -> usize;

    fn size_mut(&mut self) -> &mut usize;

    fn deep_size(&self) -> usize;

    fn deep_size_mut(&mut self) -> &mut usize;

    fn length(&self) -> S {
        self.skeleton().length()
    }

    fn deep_length(&self) -> S;

    fn deep_length_mut(&mut self) -> &mut S;

    fn append_node(&mut self, distance: S) {
        let size = self.size();
        if size == self.skeleton().size() {
            self.skeleton_mut().grow();
        }
        self.skeleton_mut().inflate_at(size, distance);
        *self.size_mut() += 1;
        *self.deep_size_mut() += 1;
        *self.deep_length_mut() += distance;
    }

    fn insert_node(&mut self, position: S) {
        if position < zero() {
            todo!()
        }
        if position >= self.length() {
            self.append_node(position - self.length());
            return
        }
        let mut traversal = ShallowTraversal::new(
            self,
            |pos| pos <= position,
            Some(|pos| pos == position)
        );
        traversal.run();
        let ShallowPosition { index, position: node_position, .. } = traversal.position();
        let mut sublist = self.skeleton_mut().get_sublist_at_mut(index).get_or_insert_default();
        sublist.insert_node(position - node_position);
        *self.deep_size_mut() += 1;
    }

    fn inflate_after(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    fn inflate_before(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    fn deflate_after(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    fn deflate_before(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    // All possible queries:
    // - first
    // - last before
    // - first at or last before
    // - last at or last before
    // - first at
    // - last at
    // - first at or first after
    // - last at or first after
    // - first after
    // - last
    //
    // TODO long term implement all of these

    fn traversal<Continue>(&self, continue_condition: Continue)
                           -> Traversal<S, Self, Continue, fn(S) -> bool>
        where Continue: Fn(S) -> bool {
        Traversal::new(self, continue_condition, None)
    }

    fn stopping_traversal<Continue, Stop>(&self, continue_condition: Continue, stop_condition: Stop)
                                          -> Traversal<S, Self, Continue, Stop>
        where Continue: Fn(S) -> bool,
              Stop: Fn(S) -> bool {
        Traversal::new(self, continue_condition, Some(stop_condition))
    }

    fn node_before<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position <= zero() {
            return None
        }
        let mut traversal = self.traversal(|pos| pos < position);
        traversal.run();
        Some(traversal.position())
    }

    fn node_at_or_before<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position < zero() {
            return None
        }
        let mut traversal = self.traversal(|pos| pos <= position);
        traversal.run();
        Some(traversal.position())
    }

    fn node_at<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position < zero() || position > self.length() {
            return None
        }
        let mut traversal = self.traversal(|pos| pos <= position);
        traversal.run();
        let result = traversal.position();
        if result.position == position {
            Some(result)
        } else {
            None
        }
    }

    fn node_at_or_after<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position > self.length() {
            return None
        }
        let mut traversal = self.traversal(|pos| pos <= position);
        traversal.run();
        let result = traversal.position();
        if result.position == position {
            Some(result)
        } else {
            traversal.next();
            Some(traversal.position())
        }
    }

    fn node_after<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position >= self.length() {
            return None
        }
        let mut traversal = self.traversal(|pos| pos <= position);
        traversal.run();
        traversal.next();
        Some(traversal.position())
    }
}