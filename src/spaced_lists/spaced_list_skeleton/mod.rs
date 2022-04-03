use std::default::default;
use std::iter;
use std::marker::PhantomData;

use num_traits::zero;

use crate::{SpacedList, Spacing};

#[derive(Clone, Eq, PartialEq)]
pub struct SpacedListSkeleton<S: Spacing, Sub: SpacedList<S>> {
    pub(crate) link_lengths: Vec<S>,
    pub(crate) sublists: Vec<Option<Sub>>,
    size: usize,
    depth: usize,
    length: S,
}

impl<S: Spacing, Sub: SpacedList<S>> Default for SpacedListSkeleton<S, Sub> {
    fn default() -> Self {
        Self {
            link_lengths: vec![],
            sublists: vec![],
            size: 0,
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
        index < self.size()
    }

    pub(crate) fn sublist_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.size()
    }

    pub(crate) fn node_index_is_in_bounds(&self, index: usize) -> bool {
        index <= self.size()
    }

    pub(crate) fn degree_is_in_bounds(&self, index: usize) -> bool {
        index < self.depth()
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }

    pub(crate) fn length(&self) -> S {
        self.length
    }

    /// Doubles this skeletons size, or increase it to one if it is zero.
    pub(crate) fn grow(&mut self) {
        if self.link_lengths.is_empty() {
            self.link_lengths.push(zero());
            self.sublists.push(None);
            self.size += 1;
        } else {
            let length = self.length();
            self.sublists.extend(iter::repeat_with(|| None).take(self.size()));
            self.link_lengths.extend(iter::repeat_with(|| S::zero()).take(self.size() - 1));
            self.link_lengths.push(length);
            self.size *= 2;
        }
        self.depth += 1;
    }

    /// Inflates the link at the specified index.
    pub(crate) fn inflate_at(&mut self, link_index: usize, amount: S) {
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            if (link_index >> degree) & 1 == 0 {
                *self.get_link_length_at_mut(link_index) += amount;
                link_index += 1 << degree;
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
    // ___________11___________,
    // _____5______,
    // __2___,     __4___,
    // _1_,  _2_,  _3_,  _2_,
    // |00|01|10|11|00|01|10|11|
    pub(crate) unsafe fn deflate_at_unchecked(&mut self, link_index: usize, amount: S) {
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            // if (link_index >> degree) & 1 == 0 {
            let bit = 1 << degree;
            if link_index & bit == 0 {
                *self.get_link_length_at_mut(link_index) -= amount;
                link_index += bit;
            }
        }
        self.length -= amount;
    }

    pub(crate) fn deflate_at(&mut self, link_index: usize, amount: S) {
        if link_index & 1 == 0 {
            // SAFETY: If the link index ends in a zero, it's a zero-degree link, so it does not
            // imply the link lengths of lower links, as non-zero-degree links do
            unsafe { self.deflate_at_unchecked(link_index, amount) }
        } else {
            let new_total_link_length = self.get_link_length_at(link_index);
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


        }
    }
}

mod display;

mod tests;