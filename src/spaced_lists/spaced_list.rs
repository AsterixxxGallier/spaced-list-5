use num_traits::zero;

use crate::{Iter, Position, SpacedListSkeleton, Spacing};
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
    fn new(
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

macro_rules! maybe_move_forwards {
    (<= $target:expr; $skeleton:expr, $link_index:expr,
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        let next_position = $position + $skeleton.get_link_length_at($link_index);
        if next_position <= $target {
            $position = next_position;
            $node_index += 1 << $degree;
            if $position == $target {
                if $skeleton.sublist_index_is_in_bounds($node_index) {
                    // TODO don't just descend into sublists like that when you have offsets
                    while let Some(sublist) = $skeleton.get_sublist_at($node_index) {
                        $node_index = 0;
                        $super_lists.push($list);
                        $list = sublist;
                    }
                }
                break;
            }
        }
    };
    (< $target:expr; $skeleton:expr, $link_index:expr,
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        let next_position = $position + $skeleton.get_link_length_at($link_index);
        if next_position < $target {
            $position = next_position;
            $node_index += 1 << $degree;
        }
    }
}

macro_rules! loop_while {
    ($cmp:tt $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        loop {
            let skeleton = $list.skeleton();
            let link_index = link_index($node_index, $degree);
            if !skeleton.link_index_is_in_bounds(link_index) {
                if $degree == 0 {
                    break;
                }
                $degree -= 1;
                continue;
            }
            maybe_move_forwards!($cmp $target; skeleton, link_index, $list, $super_lists, $degree,
                $node_index, $position);
            if $degree == 0 {
                if skeleton.sublist_index_is_in_bounds($node_index) {
                    if let Some(sublist) = skeleton.get_sublist_at($node_index) {
                        // TODO check too that position + sublist.offset < target
                        let sub_skeleton = sublist.skeleton();
                        $degree = sub_skeleton.depth() - 1;
                        $node_index = 0;
                        $super_lists.push($list);
                        $list = sublist;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                $degree -= 1;
            }
        }
    }
}

macro_rules! next {
    ($skeleton:expr, $list:ident, $super_lists:ident, $node_index:ident, $position:ident) => {
        {
            while $node_index == $list.skeleton().size() {
                if let Some(node_index) = $skeleton.index_in_super_list() {
                    $node_index = node_index;
                    $position -= $skeleton.length();
                    $list = $super_lists.pop().unwrap();
                } else {
                    panic!("Tried to move to next node but it's already the end of the list")
                };
            }

            let skeleton = $skeleton;
            let mut degree = 0;
            loop {
                if degree < $node_index.trailing_zeros() as usize {
                    break;
                }
                $position -= skeleton.get_link_length_at($node_index - 1);
                $node_index -= 1 << degree;
                degree += 1;
            }

            $node_index += 1 << degree;
            $position += skeleton.get_link_length_at($node_index - 1);
        }
    };
}

macro_rules! traverse_unchecked_with_variables {
    (< $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(< $target; $list, $super_lists, $degree, $node_index, $position);
            Some(Pos::new($super_lists, $list, $node_index, $position))
        }
    };
    (<= $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            Some(Pos::new($super_lists, $list, $node_index, $position))
        }
    };
    (== $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            if $position == $target {
                Some(Pos::new($super_lists, $list, $node_index, $position))
            } else {
                None
            }
        }
    };
    (>= $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            if $position == $target {
                Some(Pos::new($super_lists, $list, $node_index, $position))
            } else {
                next!($list.skeleton(), $list, $super_lists, $node_index, $position);
                Some(Pos::new($super_lists, $list, $node_index, $position))
            }
        }
    };
    (> $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            next!($list.skeleton(), $list, $super_lists, $node_index, $position);
            Some(Pos::new($super_lists, $list, $node_index, $position))
        }
    }
}

macro_rules! traverse_unchecked {
    ($list:expr; $cmp:tt $target:expr) => {
        {
            let mut list = $list;
            let mut super_lists = vec![];
            let mut degree = list.skeleton().depth() - 1;
            let mut node_index = 0;
            // TODO start at offset
            let mut position = zero();
            traverse_unchecked_with_variables!($cmp $target;
                list, super_lists, degree, node_index, position)
        }
    }
}

macro_rules! traverse {
    ($list:expr; < $target:expr) => {
        // TODO check if it's smaller than or equal to offset instead
        if $target <= zero() {
            None
        } else {
            traverse_unchecked!($list; < $target)
        }
    };
    ($list:expr; <= $target:expr) => {
        // TODO check if it's smaller than offset instead
        if $target < zero() {
            None
        } else {
            traverse_unchecked!($list; <= $target)
        }
    };
    ($list:expr; == $target:expr) => {
        // TODO check if it's smaller than offset instead
        if $target < zero() {
            None
        } else {
            traverse_unchecked!($list; == $target)
        }
    };
    ($list:expr; >= $target:expr) => {
        // TODO check if it's larger than offset + length instead
        if $target > $list.skeleton().length() {
            None
            // TODO replace zero() with offset
        } else if $target <= zero() {
            Some(Pos::new(vec![], $list, 0, zero()))
        } else {
            traverse_unchecked!($list; >= $target)
        }
    };
    ($list:expr; > $target:expr) => {
        // TODO check if it's larger than or equal to offset + length instead
        if $target >= $list.skeleton().length() {
            None
            // TODO replace zero() with offset
        } else if $target < zero() {
            Some(Pos::new(vec![], $list, 0, zero()))
        } else {
            traverse_unchecked!($list; > $target)
        }
    }
}

pub trait SpacedList<S: Spacing>: Default {
    fn skeleton(&self) -> &SpacedListSkeleton<S, Self>;

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self>;

    fn iter(&self) -> Iter<S, Self> {
        Iter::new(self)
    }

    // TODO add try_ versions of the methods below

    fn append_node(&mut self, distance: S) -> Position<S, Self> {
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
        Position::new(self, index, position)
    }

    fn insert_node<'a>(&'a mut self, position: S) -> Position<'a, S, Self> where S: 'a {
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