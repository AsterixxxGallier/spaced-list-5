use std::collections::HashMap;
use std::iter;

use num_traits::zero;
use paste::paste;

use crate::{SpacedList, Spacing};
use crate::skeleton::traversal::link_index;

#[derive(Clone, Eq, PartialEq)]
pub struct Skeleton<S: Spacing, Sub: SpacedList<S>> {
    link_lengths: Vec<S>,
    sublists: Vec<Option<Sub>>,
    index_in_super_list: Option<usize>,
    link_capacity: usize,
    link_size: usize,
    link_size_deep: usize,
    node_size: usize,
    node_size_deep: usize,
    depth: usize,
    length: S,
    offset: S,
}

impl<S: Spacing, Sub: SpacedList<S>> Default for Skeleton<S, Sub> {
    fn default() -> Self {
        Self {
            link_lengths: vec![],
            sublists: vec![],
            link_capacity: 0,
            depth: 0,
            length: zero(),
            offset: zero(),
            link_size: 0,
            link_size_deep: 0,
            node_size: 0,
            node_size_deep: 0,
            index_in_super_list: None,
        }
    }
}

impl<S: Spacing, Sub: SpacedList<S>> Skeleton<S, Sub> {
    accessors! {
        pub index_in_super_list: Option<usize>;
        pub mut index_in_super_list: Option<usize>;

        pub link_size: usize;
        pub mut link_size: usize;
        pub link_size_deep: usize;
        pub mut link_size_deep: usize;
        pub link_capacity: usize;
        pub node_size: usize;
        pub mut node_size: usize;
        pub node_size_deep: usize;
        pub mut node_size_deep: usize;
        pub depth: usize;
        pub length: S;
        pub offset: S;
        pub mut offset: S;

        pub index link_length: S;
        pub(crate) index mut link_length: S;
    }

    pub fn sublist_at(&self, index: usize) -> Option<&Sub> {
        self.sublists.get(index).and_then(|opt| opt.as_ref())
    }

    pub fn sublist_at_mut(&mut self, index: usize) -> Option<&mut Sub> {
        self.sublists.get_mut(index).and_then(|opt| opt.as_mut())
    }

    pub fn last_position(&self) -> S {
        self.offset() + self.length()
    }

    pub fn node_capacity(&self) -> usize {
        self.link_capacity + 1
    }

    pub fn get_or_add_sublist_at(&mut self, index: usize) -> &Sub {
        self.get_or_add_sublist_at_mut(index)
    }

    pub fn get_or_add_sublist_at_mut(&mut self, index: usize) -> &mut Sub {
        self.sublists[index].get_or_insert_with(|| {
            let mut sub = Sub::default();
            *sub.skeleton_mut().index_in_super_list_mut() = Some(index);
            sub
        })
    }

    // ╭───────────────────────────────────────────────────────────────╮
    // ├───────────────────────────────╮                               │
    // ├───────────────╮               ├───────────────╮               │
    // ├───────╮       ├───────╮       ├───────╮       ├───────╮       │
    // ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
    // ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
    // 00000   00010   00100   00110   01000   01010   01100   01110   10000
    //     00001   00011   00101   00111   01001   01011   01101   01111
    pub fn link_length_at_node(&self, index: usize) -> S {
        let mut length = self.link_length_at(index);
        for degree in 0..index.trailing_ones() {
            length -= self.link_length_at(index - (1 << degree));
        }
        length
    }

    pub fn link_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.link_size()
    }

    pub fn sublist_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.link_size()
    }

    pub fn node_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.node_size()
    }

    pub fn degree_is_in_bounds(&self, index: usize) -> bool {
        index < self.depth()
    }

    /// For unit tests only.
    #[cfg(test)]
    pub(crate) fn set_size_to_capacity(&mut self) {
        self.link_size = self.link_capacity;
        self.node_size = self.link_capacity + 1;
    }

    /// Doubles this skeletons size, or increase it to one if it is zero.
    pub fn grow(&mut self) {
        if self.link_lengths.is_empty() {
            self.link_lengths.push(zero());
            self.sublists.push(None);
            self.link_capacity += 1;
        } else {
            let length = self.length();
            self.sublists.extend(iter::repeat_with(|| None).take(self.link_capacity()));
            self.link_lengths.extend(iter::repeat_with(|| S::zero()).take(self.link_capacity() - 1));
            self.link_lengths.push(length);
            self.link_capacity *= 2;
        }
        self.depth += 1;
    }

    /// Inflates the link at the specified index.
    pub fn inflate_at(&mut self, index: usize, amount: S) {
        // TODO add inflate_at_unchecked maybe
        assert!(self.link_index_is_in_bounds(index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        for degree in 0..self.depth() {
            if index >> degree & 1 == 0 {
                let link_index = link_index(index >> degree << degree, degree);
                *self.link_length_at_mut(link_index) += amount;
            }
        }
        self.length += amount;
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
    pub unsafe fn deflate_at_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..self.depth() {
            if index >> degree & 1 == 0 {
                let link_index = link_index(index >> degree << degree, degree);
                *self.link_length_at_mut(link_index) -= amount;
            }
        }
        self.length -= amount;
    }

    pub fn deflate_at(&mut self, index: usize, amount: S) {
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
}

mod display;

mod tests;
pub(crate) mod traversal;
