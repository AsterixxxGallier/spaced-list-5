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
    pub(crate) unsafe fn deflate_at(&mut self, link_index: usize, amount: S) {
        let mut link_index = link_index;
        for degree in 0..self.depth() {
            if (link_index >> degree) & 1 == 0 {
                *self.get_link_length_at_mut(link_index) -= amount;
                link_index += 1 << degree;
            }
        }
        self.length -= amount;
    }
}

mod display;

mod tests;