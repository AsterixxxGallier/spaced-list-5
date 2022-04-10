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
    (after; $self:expr, $position:ident) => {
        traverse!(shallow; &*$self; <= $position)
    };
    (before; $self:expr, $position:ident) => {
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
        if position < self.skeleton().offset() {
            let offset = self.skeleton().offset();
            *self.skeleton_mut().offset_mut() = position;
            if self.skeleton().link_size() > 0 {
                self.skeleton_mut().inflate_at(0, offset - position);
            }
            self.insert_node(offset);
            return Position::new(vec![], self, 0, position);
        }
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

    traversal_methods! {
        before: <
        at_or_before: <=
        at: ==
        at_or_after: >=
        after: >
    }
}
