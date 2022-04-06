use std::collections::HashMap;
use std::default::default;
use std::iter;
use std::marker::PhantomData;
use std::ops::Sub;

use num_traits::zero;

use crate::{SpacedList, Spacing};

#[derive(Clone, Eq, PartialEq)]
pub struct SpacedListSkeleton<S: Spacing, Sub: SpacedList<S>> {
    pub(crate) link_lengths: Vec<S>,
    pub(crate) sublists: Vec<Option<Sub>>,
    capacity: usize,
    depth: usize,
    length: S,
}

impl<S: Spacing, Sub: SpacedList<S>> Default for SpacedListSkeleton<S, Sub> {
    fn default() -> Self {
        Self {
            link_lengths: vec![],
            sublists: vec![],
            capacity: 0,
            depth: 0,
            length: zero(),
        }
    }
}

impl<S: Spacing, Sub: SpacedList<S>> SpacedListSkeleton<S, Sub> {
    /// # Panics
    ///
    /// Panics when `index` is out of bounds.
    pub(crate) fn get_link_length_at(&self, index: usize) -> S {
        self.link_lengths[index]
    }

    /// # Panics
    ///
    /// Panics when `index` is out of bounds.
    fn get_link_length_at_mut(&mut self, index: usize) -> &mut S {
        &mut self.link_lengths[index]
    }

    /// # Panics
    ///
    /// Panics when `index` is out of bounds.
    pub(crate) fn get_sublist_at(&self, index: usize) -> &Option<Sub> {
        &self.sublists[index]
    }

    /// # Panics
    ///
    /// Panics when `index` is out of bounds.
    pub(crate) fn get_sublist_at_mut(&mut self, index: usize) -> &mut Option<Sub> {
        &mut self.sublists[index]
    }

    /// # Panics
    ///
    /// Panics when `index` is out of bounds.
    pub(crate) fn get_or_add_sublist_at(&mut self, index: usize) -> &Sub {
        self.get_or_add_sublist_at_mut(index)
    }

    /// # Panics
    ///
    /// Panics when `index` is out of bounds.
    pub(crate) fn get_or_add_sublist_at_mut(&mut self, index: usize) -> &mut Sub {
        self.sublists[index].get_or_insert_with(|| {
            let mut sub = Sub::default();
            sub.set_index_in_super_list(index);
            sub
        })
    }

    pub(crate) fn depth(&self) -> usize {
        self.depth
    }

    pub(crate) fn link_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.capacity()
    }

    pub(crate) fn sublist_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.capacity()
    }

    pub(crate) fn node_index_is_in_bounds(&self, index: usize) -> bool {
        index <= self.capacity()
    }

    pub(crate) fn degree_is_in_bounds(&self, index: usize) -> bool {
        index < self.depth()
    }

    pub(crate) fn capacity(&self) -> usize {
        self.capacity
    }

    pub(crate) fn length(&self) -> S {
        self.length
    }

    /// Doubles this skeletons size, or increase it to one if it is zero.
    pub(crate) fn grow(&mut self) {
        if self.link_lengths.is_empty() {
            self.link_lengths.push(zero());
            self.sublists.push(None);
            self.capacity += 1;
        } else {
            let length = self.length();
            self.sublists.extend(iter::repeat_with(|| None).take(self.capacity()));
            self.link_lengths.extend(iter::repeat_with(|| S::zero()).take(self.capacity() - 1));
            self.link_lengths.push(length);
            self.capacity *= 2;
        }
        self.depth += 1;
    }

    /// Inflates the link at the specified index.
    pub(crate) fn inflate_at(&mut self, link_index: usize, amount: S) {
        // TODO add inflate_at_unchecked
        assert!(self.link_index_is_in_bounds(link_index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            let bit = 1 << degree;
            if link_index & bit == 0 {
                *self.get_link_length_at_mut(link_index) += amount;
                link_index += bit;
            }
        }
        self.length += amount;
    }

    /// Deflates the link at the specified index.
    ///
    /// When deflating link lengths below zero, this causes this skeleton to assume an invalid
    /// state, which will lead to other methods behaving in undefined ways.
    // TODO add a safe wrapper / make this safe
    // example:
    //
    // assert_eq!(skeleton.link_lengths, vec![2, 2, 0, 2]);
    // skeleton.deflate_at(1, 1);
    // assert_eq!(skeleton.link_lengths, vec![2, 1, 0, 1]);
    //
    // now, the link length at index 1 is smaller than the link length at index 0, which is against
    // the rules (link 1 *contains* link 0, meaning that the distance between node 1 and node 2 is
    // now implied negative, which is illegal)
    pub(crate) unsafe fn deflate_at_unchecked(&mut self, link_index: usize, amount: S) {
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            let bit = 1 << degree;
            if link_index & bit == 0 {
                *self.get_link_length_at_mut(link_index) -= amount;
                link_index += bit;
            }
        }
        self.length -= amount;
    }

    pub(crate) fn deflate_at(&mut self, link_index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(link_index), "Link index not in bounds");
        assert!(amount >= zero(), "Cannot deflate by negative amount, explicitly inflate for that");
        // TODO check that no implied link length becomes negative
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
                    let new_total_link_length = self.get_link_length_at(link_index) - amount;
                    assert!(new_total_link_length >= zero(), "Cannot deflate a link below zero");
                    let mut sum_of_concrete_link_lengths_below = S::zero();
                    let mut link_index_below = link_index - 1;
                    for degree_below in 0..degree {
                        let link_length = overwritten_link_lengths.get(&link_index_below);
                        let link_length = match link_length {
                            Some(&link_length) => link_length,
                            None => self.get_link_length_at(link_index_below)
                        };
                        sum_of_concrete_link_lengths_below += link_length;
                        // link_index_below will not be used after the last iteration, meaning that we
                        // can ignore the underflow that can only happen then
                        link_index_below = link_index_below.wrapping_sub(1 << degree_below);
                    }
                    assert!(new_total_link_length >= sum_of_concrete_link_lengths_below,
                            "Cannot deflate a link below zero");
                }
                let link_length = self.get_link_length_at_mut(link_index);
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