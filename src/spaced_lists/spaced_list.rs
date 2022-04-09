use num_traits::zero;
use paste::paste;

use crate::{Iter, Position, Skeleton, Spacing};
use crate::spaced_lists::positions::shallow::ShallowPosition;
use crate::spaced_lists::traversal::*;

macro_rules! flate_offset_check {
    (inflate after; $self:expr, $position:expr, $amount:expr) => {
        if $position < $self.skeleton().offset() {
            *$self.skeleton_mut().offset_mut() += $amount;
            return;
        }
    };
    (inflate before; $self:expr, $position:expr, $amount:expr) => {
        if $position <= $self.skeleton().offset() {
            *$self.skeleton_mut().offset_mut() += $amount;
            return;
        }
    };
    (deflate after; $self:expr, $position:expr, $amount:expr) => {
        if $position < $self.skeleton().offset() {
            *$self.skeleton_mut().offset_mut() -= $amount;
            return;
        }
    };
    (deflate before; $self:expr, $position:expr, $amount:expr) => {
        if $position <= $self.skeleton().offset() {
            *$self.skeleton_mut().offset_mut() -= $amount;
            return;
        }
    };
}

macro_rules! flate_check {
    ($action:ident after; $self:expr, $position:expr) => {
        if $position >= $self.skeleton().length() + $self.skeleton().offset() {
            // TODO better error message
            panic!(concat!("Cannot ", stringify!($action), " out of bounds"))
        }
    };
    ($action:ident before; $self:expr, $position:expr) => {
        if $position > $self.skeleton().length() + $self.skeleton().offset() {
            // TODO better error message
            panic!(concat!("Cannot ", stringify!($action), " out of bounds"))
        }
    }
}

macro_rules! flate_position {
    (after; $self:expr, $position:expr) => {
        traverse!(shallow; &*$self; <= $position)
    };
    (before; $self:expr, $position:expr) => {
        traverse!(shallow; &*$self; < $position)
    }
}

macro_rules! flate_methods {
    {$($action:ident $pos:ident)+} => {
        paste! {
            $(fn [<$action _ $pos>](&mut self, position: S, amount: S) {
                flate_offset_check!($action $pos; self, position, amount);
                flate_check!($action $pos; self, position);
                let ShallowPosition { index, position: node_position, .. } =
                    flate_position!($pos; self, position).unwrap();
                self.skeleton_mut().[<$action _at>](index, amount);
                if let Some(sublist) = self.skeleton_mut().sublist_at_mut(index) {
                    let position_in_sublist = position - node_position;
                    if position_in_sublist < sublist.skeleton().length() {
                        sublist.[<$action _ $pos>](position_in_sublist, amount);
                    }
                }
            })+
        }
    }
}

macro_rules! traversal_methods {
    {$($pos:ident: $cmp:tt)+} => {
        paste! {
            $(fn [<node_ $pos>]<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>> where S: 'a {
                traverse!(deep; self; $cmp target)
            })+
        }
    };
}

pub trait SpacedList<S: Spacing>: Default {
    fn skeleton(&self) -> &Skeleton<S, Self>;

    fn skeleton_mut(&mut self) -> &mut Skeleton<S, Self>;

    fn iter(&self) -> Iter<S, Self> {
        Iter::new(self)
    }

    // TODO add try_ versions of the methods below

    fn append_node(&mut self, distance: S) -> Position<S, Self> {
        // TODO possibly, there might be future problems when increasing the length of a sublist
        //  beyond the link length from the node the sublist is positioned after to the node the
        //  sublist is positioned before, but this should never happen because sublists are only
        //  accessible from within this crate
        // TODO add edge case support for first addition that sets offset
        let link_size = self.skeleton().link_size();
        let node_size = self.skeleton().node_size();
        if node_size == 0 {
            *self.skeleton_mut().node_size_mut() += 1;
            *self.skeleton_mut().node_size_deep_mut() += 1;
            *self.skeleton_mut().offset_mut() = distance;
            return Position::new(vec![], self, 0, distance);
        }
        if node_size == self.skeleton().node_capacity() {
            self.skeleton_mut().grow();
        }
        *self.skeleton_mut().link_size_mut() += 1;
        *self.skeleton_mut().link_size_deep_mut() += 1;
        *self.skeleton_mut().node_size_mut() += 1;
        *self.skeleton_mut().node_size_deep_mut() += 1;
        self.skeleton_mut().inflate_at(link_size, distance);
        let index = self.skeleton().link_size() - 1;
        let position = self.skeleton().length();
        Position::new(vec![], self, index, position)
    }

    fn insert_node<'a>(&'a mut self, position: S) -> Position<'a, S, Self> where S: 'a {
        // TODO check if smaller than offset instead
        if position <= self.skeleton().offset() {
            let offset = self.skeleton().offset();
            *self.skeleton_mut().offset_mut() = position;
            if self.skeleton().link_size() > 0 {
                self.skeleton_mut().inflate_at(0, offset - position);
            }
            self.insert_node(offset);
            return Position::new(vec![], self, 0, position);
        }
        // TODO check if larger than or equal to length + offset instead
        if position >= self.skeleton().length() + self.skeleton().offset() {
            return self.append_node(position - self.skeleton().length() - self.skeleton().offset());
        }
        let ShallowPosition { index, position: node_position, .. } =
            traverse!(shallow; &*self; <= position).unwrap();
        *self.skeleton_mut().link_size_deep_mut() += 1;
        *self.skeleton_mut().node_size_deep_mut() += 1;
        let sublist = self.skeleton_mut().get_or_add_sublist_at_mut(index);
        sublist.insert_node(position - node_position)
    }

    flate_methods! {
        inflate after
        inflate before
        deflate after
        deflate before
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

    // fn node_before<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
    //     where
    //         S: 'a,
    // {
    //     if target <= self.skeleton().offset() {
    //         None
    //     } else {
    //         {
    //             let mut list = self;
    //             let mut super_lists = Vec::new();
    //             let mut degree = list.skeleton().depth() - 1;
    //             let mut node_index = 0;
    //             let mut position = list.skeleton().offset();
    //             {
    //                 loop {
    //                     let skeleton = list.skeleton();
    //                     let link_index = link_index(node_index, degree);
    //                     if !skeleton.link_index_is_in_bounds(link_index) {
    //                         if degree == 0 {
    //                             break;
    //                         }
    //                         degree -= 1;
    //                         continue;
    //                     }
    //                     let next_position = position + skeleton.link_length_at(link_index);
    //                     if next_position < target {
    //                         position = next_position;
    //                         node_index += 1 << degree;
    //                     };
    //                     if degree == 0 {
    //                         if skeleton.sublist_index_is_in_bounds(node_index) {
    //                             if let Some(sublist) = skeleton.sublist_at(node_index) {
    //                                 let sub_skeleton = sublist.skeleton();
    //                                 if position + sub_skeleton.offset() < target {
    //                                     if sub_skeleton.link_size() == 0 {
    //                                         node_index = 0;
    //                                         position += sub_skeleton.offset();
    //                                         super_lists.push(list);
    //                                         list = sublist;
    //                                         break;
    //                                     }
    //                                     degree = sub_skeleton.depth() - 1;
    //                                     node_index = 0;
    //                                     position += sub_skeleton.offset();
    //                                     super_lists.push(list);
    //                                     list = sublist;
    //                                     continue;
    //                                 }
    //                             }
    //                         }
    //                         break;
    //                     } else {
    //                         degree -= 1;
    //                     };
    //                 }
    //                 Some(Position::new(super_lists, list, node_index, position))
    //             }
    //         }
    //     }
    // }
    traversal_methods! {
        before: <
        at_or_before: <=
        at: ==
        at_or_after: >=
        after: >
    }
    // fn node_after<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
    //     where
    //         S: 'a,
    // {
    //     if target >= self.skeleton().length() + self.skeleton().offset() {
    //         None
    //     } else if target < self.skeleton().offset() {
    //         Some(Position::new(
    //             Vec::new(),
    //             self,
    //             0,
    //             self.skeleton().offset(),
    //         ))
    //     } else {
    //         {
    //             let mut list = self;
    //             let mut super_lists = Vec::new();
    //             let mut degree = list.skeleton().depth() - 1;
    //             let mut node_index = 0;
    //             let mut position = list.skeleton().offset();
    //             {
    //                 loop {
    //                     let skeleton = list.skeleton();
    //                     let link_index = link_index(node_index, degree);
    //                     if !skeleton.link_index_is_in_bounds(link_index) {
    //                         if degree == 0 {
    //                             break;
    //                         }
    //                         degree -= 1;
    //                         continue;
    //                     }
    //                     let next_position = position + skeleton.link_length_at(link_index);
    //                     if next_position <= target {
    //                         position = next_position;
    //                         node_index += 1 << degree;
    //                         if position == target {
    //                             break;
    //                         }
    //                     };
    //                     if degree == 0 {
    //                         if skeleton.sublist_index_is_in_bounds(node_index) {
    //                             if let Some(sublist) = skeleton.sublist_at(node_index) {
    //                                 let sub_skeleton = sublist.skeleton();
    //                                 if position + sub_skeleton.offset() <= target {
    //                                     if sub_skeleton.link_size() == 0 {
    //                                         node_index = 0;
    //                                         position += sub_skeleton.offset();
    //                                         super_lists.push(list);
    //                                         list = sublist;
    //                                         break;
    //                                     }
    //                                     degree = sub_skeleton.depth() - 1;
    //                                     node_index = 0;
    //                                     position += sub_skeleton.offset();
    //                                     super_lists.push(list);
    //                                     list = sublist;
    //                                     continue;
    //                                 }
    //                             }
    //                         }
    //                         break;
    //                     } else {
    //                         degree -= 1;
    //                     };
    //                 }
    //                 'next: {
    //                     // TODO move this if up in macros too!
    //                     let skeleton = list.skeleton();
    //                     if skeleton.sublist_index_is_in_bounds(node_index) {
    //                         if let Some(sublist) = skeleton.sublist_at(node_index) {
    //                             let sub_skeleton = sublist.skeleton();
    //                             node_index = 0;
    //                             position += sub_skeleton.offset();
    //                             super_lists.push(list);
    //                             list = sublist;
    //                             break 'next;
    //                         }
    //                     }
    //                     while node_index == list.skeleton().link_size() {
    //                         if let Some(new_index) = list.skeleton().index_in_super_list() {
    //                             node_index = new_index;
    //                             // TODO add offset in macros too!
    //                             position -= list.skeleton().length() + list.skeleton().offset();
    //                             list = super_lists.pop().unwrap();
    //                             continue;
    //                         }
    //                         panic!();
    //                     }
    //                     let skeleton = list.skeleton();
    //                     let mut degree = 0;
    //                     loop {
    //                         if degree < node_index.trailing_zeros() as usize {
    //                             break;
    //                         }
    //                         position -= skeleton.link_length_at(node_index - 1);
    //                         node_index -= 1 << degree;
    //                         degree += 1;
    //                     }
    //                     node_index += 1 << degree;
    //                     position += skeleton.link_length_at(node_index - 1);
    //                 };
    //                 Some(Position::new(super_lists, list, node_index, position))
    //             }
    //         }
    //     }
    // }
}
