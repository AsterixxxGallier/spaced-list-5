use std::iter;
use num_traits::zero;
use paste::paste;

use crate::{Iter, Position, Spacing};
use crate::positions::shallow::ShallowPosition;
use crate::traversal::*;

macro_rules! flate_cmp {
    (after; $a:expr, $b:expr) => { $a < $b };
    (before; $a:expr, $b:expr) => { $a <= $b }
}

macro_rules! flate_offset_check {
    (inflate after; $self:expr, $position:expr, $amount:expr) => {
        if $position < $self.offset() {
            *$self.offset_mut() += $amount;
            return;
        }
    };
    (inflate before; $self:expr, $position:expr, $amount:expr) => {
        if $position <= $self.offset() {
            *$self.offset_mut() += $amount;
            return;
        }
    };
    (deflate after; $self:expr, $position:expr, $amount:expr) => {
        if $position < $self.offset() {
            *$self.offset_mut() -= $amount;
            return;
        }
    };
    (deflate before; $self:expr, $position:expr, $amount:expr) => {
        if $position <= $self.offset() {
            *$self.offset_mut() -= $amount;
            return;
        }
    };
}

macro_rules! flate_check {
    ($action:ident after; $self:expr, $position:expr) => {
        if $position >= $self.last_position() {
            panic!(concat!("Cannot ", stringify!($action),
                " after the given position, as that position is at or after this list"));
        }
    };
    ($action:ident before; $self:expr, $position:expr) => {
        if $position > $self.last_position() {
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
                self.[<$action _at>](index, amount);
                if let Some(sublist) = self.sublist_at_mut(index) {
                    let position_in_sublist = position - node_position;
                    if flate_cmp!($pos; position_in_sublist, sublist.last_position()) {
                        sublist.[<$action _ $pos>](position_in_sublist, amount);
                    }
                }
            })+
        }
    }
}

macro_rules! traversal_methods {
    (@$pos:ident: $cmp:tt) => {
        paste! {
            fn [<node_ $pos>]<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
                where S: 'a,
                      Self: SpacedList<S> {
                traverse!(node; deep; self; $cmp target)
            }
        }
    };
    () => {
        for_all_traversals!(traversal_methods @);
    }
}

pub trait CrateSpacedList<S: Spacing>: Default {
    trait_accessors! {
        index_in_super_list: Option<usize>;
        mut index_in_super_list: Option<usize>;
        ref link_lengths: Vec<S>;
        mut link_lengths: Vec<S>;
        ref sublists: Vec<Option<Self>>;
        mut sublists: Vec<Option<Self>>;
        link_size: usize;
        mut link_size: usize;
        link_size_deep: usize;
        mut link_size_deep: usize;
        link_capacity: usize;
        mut link_capacity: usize;
        node_size: usize;
        mut node_size: usize;
        node_size_deep: usize;
        mut node_size_deep: usize;
        depth: usize;
        mut depth: usize;
        length: S;
        mut length: S;
        offset: S;
        mut offset: S;
        index link_length: S;
        index mut link_length: S;
    }

    fn sublist_at(&self, index: usize) -> Option<&Self> {
        self.sublists().get(index).and_then(|opt| opt.as_ref())
    }

    fn sublist_at_mut(&mut self, index: usize) -> Option<&mut Self> {
        self.sublists_mut().get_mut(index).and_then(|opt| opt.as_mut())
    }

    fn get_or_add_sublist_at(&mut self, index: usize) -> &Self {
        self.get_or_add_sublist_at_mut(index)
    }

    fn get_or_add_sublist_at_mut(&mut self, index: usize) -> &mut Self {
        self.sublists_mut()[index].get_or_insert_with(|| {
            let mut sub = Self::default();
            *sub.index_in_super_list_mut() = Some(index);
            sub
        })
    }

    fn last_position(&self) -> S {
        self.offset() + self.length()
    }

    fn node_capacity(&self) -> usize {
        self.link_capacity() + 1
    }

    // ╭───────────────────────────────────────────────────────────────╮
    // ├───────────────────────────────╮                               │
    // ├───────────────╮               ├───────────────╮               │
    // ├───────╮       ├───────╮       ├───────╮       ├───────╮       │
    // ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
    // ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
    // 00000   00010   00100   00110   01000   01010   01100   01110   10000
    //     00001   00011   00101   00111   01001   01011   01101   01111
    fn link_length_at_node(&self, index: usize) -> S {
        let mut length = self.link_length_at(index);
        for degree in 0..index.trailing_ones() {
            length -= self.link_length_at(index - (1 << degree));
        }
        length
    }

    fn link_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.link_size()
    }

    fn sublist_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.link_size()
    }

    fn node_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.node_size()
    }

    fn degree_is_in_bounds(&self, index: usize) -> bool {
        index < self.depth()
    }

    /// For unit tests only.
    #[cfg(test)]
    fn set_size_to_capacity(&mut self) {
        *self.link_size_mut() = self.link_capacity();
        *self.node_size_mut() = self.link_capacity() + 1;
    }

    /// Doubles this skeletons size, or increase it to one if it is zero.
    fn grow(&mut self) {
        if self.link_lengths().is_empty() {
            self.link_lengths_mut().push(zero());
            self.sublists_mut().push(None);
            *self.link_capacity_mut() += 1;
        } else {
            let length = self.length();
            let link_capacity = self.link_capacity();
            self.sublists_mut().extend(iter::repeat_with(|| None).take(link_capacity));
            self.link_lengths_mut().extend(iter::repeat_with(|| S::zero()).take(link_capacity - 1));
            self.link_lengths_mut().push(length);
            *self.link_capacity_mut() *= 2;
        }
        *self.depth_mut() += 1;
    }

    /// Inflates the link at the specified index.
    fn inflate_at(&mut self, index: usize, amount: S) {
        // TODO add inflate_at_unchecked maybe
        assert!(self.link_index_is_in_bounds(index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        for degree in 0..self.depth() {
            if index >> degree & 1 == 0 {
                let link_index = link_index(index >> degree << degree, degree);
                *self.link_length_at_mut(link_index) += amount;
            }
        }
        *self.length_mut() += amount;
    }

    /// Deflates the link at the specified index.
    ///
    /// When deflating link lengths below zero, this causes this skeleton to assume an invalid
    /// state, which will lead to other methods behaving in undefined ways.
    // example:
    //
    // assert_eq!(skeleton.link_lengths, vec![2, 2, 0, 2]);
    // skeleton.deflate_at(1, 1);
    // assert_eq!(skeleton.link_lengths, vec![2, 1, 0, 1]);
    //
    // now, the link length at index 1 is smaller than the link length at index 0, which is against
    // the rules (link 1 *contains* link 0, meaning that the distance between node 1 and node 2 is
    // now implied negative, which is illegal)
    // TODO double check this should actually be marked as unsafe
    unsafe fn deflate_at_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..self.depth() {
            if index >> degree & 1 == 0 {
                let link_index = link_index(index >> degree << degree, degree);
                *self.link_length_at_mut(link_index) -= amount;
            }
        }
        *self.length_mut() -= amount;
    }

    fn deflate_at(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot deflate by negative amount, explicitly inflate for that");
        for degree in 0..self.depth() {
            if index >> degree & 1 == 0 {
                let link_index = link_index(index >> degree << degree, degree);
                assert!(self.link_length_at_node(link_index) >= amount,
                        "Deflating at this index would deflate a link below zero");
            }
        }
        unsafe {
            self.deflate_at_unchecked(index, amount);
        }
    }

    fn iter(&self) -> Iter<S, Self> where Self: SpacedList<S> {
        Iter::new(self)
    }

    // TODO add try_ versions of the methods below

    fn append_node(&mut self, distance: S) -> Position<S, Self> where Self: SpacedList<S> {
        // possibly, there might be future problems when increasing the length of a sublist beyond
        // the link length from the node the sublist is positioned after to the node the sublist is
        // positioned before, but this should never happen because sublists are only accessible from
        // within this crate and are only ever inserted into through insert methods, which traverse
        // the super list first and will skip all sublists where this could become a problem
        let link_size = self.link_size();
        let node_size = self.node_size();
        if node_size == 0 {
            *self.node_size_mut() += 1;
            *self.node_size_deep_mut() += 1;
            *self.offset_mut() = distance;
            return Position::new(vec![], self, 0, distance);
        }
        if node_size == self.node_capacity() {
            self.grow();
        }
        *self.link_size_mut() += 1;
        *self.link_size_deep_mut() += 1;
        *self.node_size_mut() += 1;
        *self.node_size_deep_mut() += 1;
        self.inflate_at(link_size, distance);
        let position = self.length();
        Position::new(vec![], self, link_size, position)
    }

    fn insert_node<'a>(&'a mut self, position: S) -> Position<'a, S, Self>
        where S: 'a,
              Self: SpacedList<S> {
        if position < self.offset() {
            let offset = self.offset();
            *self.offset_mut() = position;
            if self.link_size() > 0 {
                self.inflate_after(self.offset(), offset - position);
            }
            self.insert_node(offset);
            return Position::new(vec![], self, 0, position);
        }
        if position >= self.last_position() {
            return self.append_node(position - self.last_position());
        }
        let ShallowPosition { index, position: node_position, .. } =
            traverse!(node; shallow; &*self; <= position).unwrap();
        *self.link_size_deep_mut() += 1;
        *self.node_size_deep_mut() += 1;
        let sublist = self.get_or_add_sublist_at_mut(index);
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

    traversal_methods!();
}

pub trait SpacedList<S: Spacing>: CrateSpacedList<S> {}

pub(crate) mod hollow_spaced_list;

pub(crate) mod filled_spaced_list;
mod tests;
