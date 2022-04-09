use num_traits::zero;

use crate::{Iter, SpacedListSkeleton, Spacing};
use crate::spaced_lists::traversal::node::*;
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
    ($cmp:tt, $list:expr, $position:expr) => {
        {
            let mut traversal = shallow_traversal!($cmp, $list, $position);
            traversal.run();
            traversal.position()
        }
    }
}

// ╭───────────────────────────────────────────────────────────────╮
// ├───────────────────────────────╮                               │
// ├───────────────╮               ├───────────────╮               │
// ├───────╮       ├───────╮       ├───────╮       ├───────╮       │
// ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
// ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
// 00000   00010   00100   00110   01000   01010   01100   01110   10000
//     00001   00011   00101   00111   01001   01011   01101   01111
//
// backwards structure, does not make a lot of sense unfortunately:
// ╭───────────────────────────────────────────────────────────────╮
// │                               ╭───────────────────────────────┤
// │               ╭───────────────┤               ╭───────────────┤
// │       ╭───────┤       ╭───────┤       ╭───────┤       ╭───────┤
// │   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤
// ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
// 00000   00010   00100   00110   01000   01010   01100   01110   10000
//     00001   00011   00101   00111   01001   01011   01101   01111

const fn link_index(node_index: usize, degree: usize) -> usize {
    node_index + (1 << degree) - 1
}

pub struct Pos<'list, S: Spacing, List: SpacedList<S>> {
    super_lists: Vec<&'list List>,
    list: &'list List,
    index: usize,
    position: S,
}

impl<'list, S: Spacing, List: SpacedList<S>> Pos<'list, S, List> {
    pub(crate) fn new(
        super_lists: Vec<&'list List>,
        list: &'list List,
        index: usize,
        position: S,
    ) -> Self {
        Pos {
            super_lists,
            list,
            index,
            position,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }
}

pub trait SpacedList<S: Spacing>: Default {
    fn skeleton(&self) -> &SpacedListSkeleton<S, Self>;

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self>;

    fn iter(&self) -> Iter<S, Self> {
        Iter::new(self)
    }

    // TODO add try_ versions of the methods below

    fn append_node(&mut self, distance: S) -> Pos<S, Self> {
        // TODO possibly, there might be future problems when increasing the length of a sublist
        //  beyond the link length from the node the sublist is positioned after to the node the
        //  sublist is positioned before, but this should never happen because sublists are only
        //  accessible from within this crate
        let size = self.skeleton().size();
        if size == self.skeleton().capacity() {
            self.skeleton_mut().grow();
        }
        self.skeleton_mut().inflate_at(size, distance);
        let index = self.skeleton().size();
        let position = self.skeleton().length();
        *self.skeleton_mut().size_mut() += 1;
        *self.skeleton_mut().deep_size_mut() += 1;
        Pos::new(vec![], self, index, position)
    }

    fn insert_node<'a>(&'a mut self, position: S) -> Pos<'a, S, Self> where S: 'a {
        if position < zero() {
            todo!()
        }
        if position >= self.skeleton().length() {
            return self.append_node(position - self.skeleton().length());
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<=, self, position);
        *self.skeleton_mut().deep_size_mut() += 1;
        let sublist = self.skeleton_mut().get_or_add_sublist_at_mut(index);
        sublist.insert_node(position - node_position)
    }

    fn inflate_after(&mut self, position: S, amount: S) {
        if position < zero() || position >= self.skeleton().length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<=, self, position);
        self.skeleton_mut().inflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.skeleton().length() {
                sublist.inflate_after(position_in_sublist, amount);
            }
        }
    }

    fn inflate_before(&mut self, position: S, amount: S) {
        if position <= zero() || position > self.skeleton().length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<, self, position);
        self.skeleton_mut().inflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.skeleton().length() {
                sublist.inflate_before(position_in_sublist, amount);
            }
        }
    }

    fn deflate_after(&mut self, position: S, amount: S) {
        if position < zero() || position >= self.skeleton().length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<=, self, position);
        self.skeleton_mut().deflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.skeleton().length() {
                sublist.deflate_after(position_in_sublist, amount);
            }
        }
    }

    fn deflate_before(&mut self, position: S, amount: S) {
        if position <= zero() || position > self.skeleton().length() {
            todo!()
        }
        let ShallowPosition { index, position: node_position, .. } =
            shallow_traversal_position!(<, self, position);
        self.skeleton_mut().deflate_at(index, amount);
        if let Some(sublist) = self.skeleton_mut().get_sublist_at_mut(index) {
            let position_in_sublist = position - node_position;
            if position_in_sublist < sublist.skeleton().length() {
                sublist.deflate_before(position_in_sublist, amount);
            }
        }
    }

    /*All possible queries:
    - first
    - last before
    - first at or last before
    - last at or last before
    - first at
    - last at
    - first at or first after
    - last at or first after
    - first after
    - last

    TODO long term implement all of these*/

    fn node_before<'a>(&'a self, position: S) -> Option<Pos<'a, S, Self>> where S: 'a {
        traverse!(self; < position)
    }

    fn node_at_or_before<'a>(&'a self, position: S) -> Option<Pos<'a, S, Self>> where S: 'a {
        traverse!(self; <= position)
    }

    fn node_at<'a>(&'a self, position: S) -> Option<Pos<'a, S, Self>> where S: 'a {
        traverse!(self; == position)
    }

    fn node_at_or_after<'a>(&'a self, position: S) -> Option<Pos<'a, S, Self>> where S: 'a {
        traverse!(self; >= position)
    }

    fn node_after<'a>(&'a self, position: S) -> Option<Pos<'a, S, Self>> where S: 'a {
        traverse!(self; > position)
    }
}