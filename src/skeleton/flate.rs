use num_traits::zero;
use crate::skeleton::{link_index, Skeleton};
use crate::Spacing;

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    fn inflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                self.links[link_index(index, degree)] += amount;
            }
        }
        self.length += amount
    }

    fn inflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        self.inflate_unchecked(index, amount)
    }

    fn deflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                self.links[link_index(index, degree)] -= amount;
            }
        }
        self.length -= amount
    }

    fn deflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero(), "Cannot deflate by negative amount, explicitly inflate for that");
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                assert!(self.link(link_index(index, degree)) >= amount,
                        "Deflating at this index would deflate a link below zero");
            }
        }
        self.inflate_unchecked(index, amount)
    }

    fn inflate_after_offset(&mut self, amount: S) {
        if !self.links.is_empty() {
            self.inflate(0, amount);
            if let Some(sub) = self.sub(0) {
                sub.borrow_mut().offset += amount;
            }
        }
    }
}