use paste::paste;

use crate::{Iter, Position, Skeleton, Spacing};
use crate::positions::shallow::ShallowPosition;
use crate::skeleton::traversal::*;

macro_rules! flate_cmp {
    (after; $a:expr, $b:expr) => { $a < $b };
    (before; $a:expr, $b:expr) => { $a <= $b }
}

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
        if $position >= $self.skeleton().last_position() {
            panic!(concat!("Cannot ", stringify!($action),
                " after the given position, as that position is at or after this list"));
        }
    };
    ($action:ident before; $self:expr, $position:expr) => {
        if $position > $self.skeleton().last_position() {
            panic!(concat!("Cannot ", stringify!($action),
                " before the given position, as that position is after this list"));
        }
    }
}

macro_rules! flate_position {
    (after; $self:expr, $position:ident) => {
        traverse!(node; shallow; &*$self; <= $position)
    };
    (before; $self:expr, $position:ident) => {
        traverse!(node; shallow; &*$self; < $position)
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
                    if flate_cmp!($pos; position_in_sublist, sublist.skeleton().last_position()) {
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
                traverse!(node; deep; self; $cmp target)
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
        // possibly, there might be future problems when increasing the length of a sublist beyond
        // the link length from the node the sublist is positioned after to the node the sublist is
        // positioned before, but this should never happen because sublists are only accessible from
        // within this crate and are only ever inserted into through insert methods, which traverse
        // the super list first and will skip all sublists where this could become a problem
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
        let position = self.skeleton().length();
        Position::new(vec![], self, link_size, position)
    }

    fn insert_node<'a>(&'a mut self, position: S) -> Position<'a, S, Self> where S: 'a {
        if position < self.skeleton().offset() {
            let offset = self.skeleton().offset();
            *self.skeleton_mut().offset_mut() = position;
            if self.skeleton().link_size() > 0 {
                self.inflate_after(self.skeleton().offset(), offset - position);
            }
            self.insert_node(offset);
            return Position::new(vec![], self, 0, position);
        }
        if position >= self.skeleton().last_position() {
            return self.append_node(position - self.skeleton().last_position());
        }
        let ShallowPosition { index, position: node_position, .. } =
            traverse!(node; shallow; &*self; <= position).unwrap();
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

pub(crate) mod hollow_spaced_list;

pub(crate) mod filled_spaced_list;
