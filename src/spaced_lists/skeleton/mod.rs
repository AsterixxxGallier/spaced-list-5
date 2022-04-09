use std::collections::HashMap;
use std::iter;

use num_traits::zero;
use paste::paste;

use crate::{SpacedList, Spacing};

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
        index mut link_length: S;
        pub index ref sublist: Option<Sub>;
        pub index mut sublist: Option<Sub>;
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
    pub fn inflate_at(&mut self, link_index: usize, amount: S) {
        // TODO add inflate_at_unchecked maybe
        assert!(self.link_index_is_in_bounds(link_index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            let bit = 1 << degree;
            if link_index & bit == 0 {
                *self.link_length_at_mut(link_index) += amount;
                link_index += bit;
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
    pub unsafe fn deflate_at_unchecked(&mut self, link_index: usize, amount: S) {
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            let bit = 1 << degree;
            if link_index & bit == 0 {
                *self.link_length_at_mut(link_index) -= amount;
                link_index += bit;
            }
        }
        self.length -= amount;
    }

    pub fn deflate_at(&mut self, link_index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(link_index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot deflate by negative amount, explicitly inflate for that");
        // concrete link lengths are those for which link_index.trailing_ones() == degree
        // implied link lengths are those which are below a concrete link
        // therefore, we need to check that for all concrete links that we touch, the implied
        // links below them do not become negative by this inflation.
        // the total link length of a non-zero-degree concrete link consists of the sum of
        // as many concrete link lengths as link_index.trailing_ones() and a single, zero-degree
        // implied link length. making sure that that single zero-degree implied link length
        // does not become negative is the goal.
        // total_link_length = sum_of_concrete_link_lengths_below + implied_link_length
        // implied_link_length = total_link_length - sum_of_concrete_link_lengths_below
        // requirement: implied_link_length >= 0
        // therefore, total_link_length - sum_of_concrete_link_lengths_below >= 0
        // finally, total_link_length >= sum_of_concrete_link_lengths_below
        // ___________________11___________________,
        // _________5__________,
        // _____2____,         _____4____,
        // __1__,    __2__,    __3__,    __2__,
        // |0000|0001|0010|0011|0100|0101|0110|0111|
        let mut overwritten_link_lengths = HashMap::with_capacity(self.depth);
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            let bit = 1 << degree;
            if link_index & bit == 0 {
                if link_index > 0 {
                    let new_total_link_length = self.link_length_at(link_index) - amount;
                    assert!(new_total_link_length >= zero(), "Cannot deflate a link below zero");
                    let mut sum_of_concrete_link_lengths_below = S::zero();
                    let mut link_index_below = link_index - 1;
                    for degree_below in 0..degree {
                        let link_length = overwritten_link_lengths.get(&link_index_below);
                        let link_length = match link_length {
                            Some(&link_length) => link_length,
                            None => self.link_length_at(link_index_below)
                        };
                        sum_of_concrete_link_lengths_below += link_length;
                        // link_index_below will not be used after the last iteration, meaning that we
                        // can ignore the underflow that can only happen then
                        link_index_below = link_index_below.wrapping_sub(1 << degree_below);
                    }
                    assert!(new_total_link_length >= sum_of_concrete_link_lengths_below,
                            "Cannot deflate a link below zero");
                }
                let link_length = self.link_length_at_mut(link_index);
                overwritten_link_lengths.insert(link_index, *link_length);
                *link_length -= amount;
                link_index += bit;
            }
        }
        self.length -= amount;
    }
}

mod display;

mod tests;