use std::marker::PhantomData;
use std::ops::Neg;

use num_traits::zero;

use crate::{Position, SpacedListSkeleton, Spacing, Todo};
use crate::spaced_lists::traversal::node::Traversal;
use crate::spaced_lists::traversal::shallow::{ShallowPosition, ShallowTraversal};

macro_rules! shallow_traversal {
    (<=, $list:expr, $position:expr) => {
        ShallowTraversal::new(
            $list,
            |pos| pos <= $position,
            Some(|pos| pos == $position)
        )
    };
    (<, $list:expr, $position:expr) => {
        ShallowTraversal::new(
            $list,
            |pos| pos < $position,
            None::<fn(_) -> _>
        )
    }
}

macro_rules! shallow_traversal_position {
    (<=, $list:expr, $position:expr) => {
        {
            let mut traversal = shallow_traversal!(<=, $list, $position);
            traversal.run();
            traversal.into_position()
        }
    };
    (<, $list:expr, $position:expr) => {
        {
            let mut traversal = shallow_traversal!(<, $list, $position);
            traversal.run();
            traversal.into_position()
        }
    }
}

pub trait SpacedList<S: Spacing>: Default {
    fn index_in_super_list(&self) -> Option<usize>;

    fn set_index_in_super_list(&mut self, index: usize);

    fn skeleton(&self) -> &SpacedListSkeleton<S, Self>;

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self>;

    fn size(&self) -> usize;

    fn size_mut(&mut self) -> &mut usize;

    fn deep_size(&self) -> usize;

    // FIXME this seems to get ridiculously large under some circumstances
    fn deep_size_mut(&mut self) -> &mut usize;

    fn length(&self) -> S {
        self.skeleton().length()
    }

    // TODO add try_ versions of the methods below

    fn append_node(&mut self, distance: S) {
        // TODO possibly, there might be future problems when increasing the length of a sublist
        //  beyond the link length from the node the sublist is positioned after to the node the
        //  sublist is positioned before, but this should never happen because sublists are only
        //  accessible from within this crate
        let size = self.size();
        if size == self.skeleton().capacity() {
            self.skeleton_mut().grow();
        }
        self.skeleton_mut().inflate_at(size, distance);
        *self.size_mut() += 1;
        *self.deep_size_mut() += 1;
    }

    fn insert_node(&mut self, position: S) {
        if position < zero() {
            todo!()
        }
        if position >= self.length() {
            self.append_node(position - self.length());
            return;
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<=, self, position);
        let mut sublist = self.skeleton_mut().get_or_add_sublist_at_mut(index);
        sublist.insert_node(position - node_position);
        *self.deep_size_mut() += 1;
    }

    fn inflate_after(&mut self, position: S, amount: S) {
        if position < zero() || position >= self.length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<=, self, position);
        self.skeleton_mut().inflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.length() {
                sublist.inflate_after(position_in_sublist, amount);
            }
        }
    }

    fn inflate_before(&mut self, position: S, amount: S) {
        if position <= zero() || position > self.length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<, self, position);
        self.skeleton_mut().inflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.length() {
                sublist.inflate_before(position_in_sublist, amount);
            }
        }
    }

    fn deflate_after(&mut self, position: S, amount: S) {
        if position < zero() || position >= self.length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<=, self, position);
        self.skeleton_mut().deflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.length() {
                sublist.deflate_after(position_in_sublist, amount);
            }
        }
    }

    fn deflate_before(&mut self, position: S, amount: S) {
        if position <= zero() || position > self.length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<, self, position);
        self.skeleton_mut().deflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.length() {
                sublist.deflate_before(position_in_sublist, amount);
            }
        }
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
            return None;
        }
        let mut traversal = self.traversal(|pos| pos < position);
        traversal.run();
        Some(traversal.position())
    }

    fn node_at_or_before<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position < zero() {
            return None;
        }
        let mut traversal =
            self.stopping_traversal(|pos| pos <= position, |pos| pos == position);
        traversal.run();
        Some(traversal.position())
    }

    fn node_at<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position < zero() || position > self.length() {
            return None;
        }
        let mut traversal =
            self.stopping_traversal(|pos| pos <= position, |pos| pos == position);
        traversal.run();
        let result = traversal.position();
        if result.position() == position {
            Some(result)
        } else {
            None
        }
    }

    fn node_at_or_after<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position > self.length() {
            return None;
        }
        if position <= zero() {
            return Some(Position::new(self, 0, zero(), 0));
        }
        let mut traversal =
            self.stopping_traversal(|pos| pos <= position, |pos| pos == position);
        traversal.run();
        let result = traversal.position();
        if result.position() == position {
            Some(result)
        } else {
            traversal.next().unwrap();
            Some(traversal.position())
        }
    }

    fn node_after<'a>(&'a self, position: S) -> Option<Position<'a, S, Self>> where S: 'a {
        if position >= self.length() {
            return None;
        }
        if position < zero() {
            return Some(Position::new(self, 0, zero(), 0));
        }
        let mut traversal =
            self.stopping_traversal(|pos| pos <= position, |pos| pos == position);
        traversal.run();
        traversal.next().unwrap();
        Some(traversal.position())
    }
}