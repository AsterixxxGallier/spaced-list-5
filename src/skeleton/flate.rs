use std::cell::RefCell;
use std::rc::Rc;

use num_traits::zero;

use crate::skeleton::{link_index, Skeleton};
use crate::Spacing;

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    pub fn inflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) {
        if position < this.borrow().offset {
            this.borrow_mut().offset += amount;
            return;
        }
        assert!(position < this.borrow().last_position(),
                "Cannot inflate after the given position, as that position is at or after this list");
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        this.borrow_mut().inflate(result.index, amount);
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub < sub.borrow().last_position() {
                Self::inflate_after(sub, position_in_sub, amount);
            }
        }
    }

    pub fn inflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) {
        if position <= this.borrow().offset {
            this.borrow_mut().offset += amount;
            return;
        }
        assert!(position <= this.borrow().last_position(),
                "Cannot inflate before the given position, as that position is after this list");
        let result =
            Self::shallow_before(this.clone(), position).unwrap();
        this.borrow_mut().inflate(result.index, amount);
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub <= sub.borrow().last_position() {
                Self::inflate_before(sub, position_in_sub, amount);
            }
        }
    }

    pub fn deflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) {
        if position < this.borrow().offset {
            this.borrow_mut().offset -= amount;
            return;
        }
        assert!(position < this.borrow().last_position(),
                "Cannot deflate after the given position, as that position is at or after this list");
        let result =
            Self::shallow_at_or_before(this.clone(), position).unwrap();
        this.borrow_mut().deflate(result.index, amount);
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub < sub.borrow().last_position() {
                Self::deflate_after(sub, position_in_sub, amount);
            }
        }
    }

    pub fn deflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) {
        if position <= this.borrow().offset {
            this.borrow_mut().offset -= amount;
            return;
        }
        assert!(position <= this.borrow().last_position(),
                "Cannot deflate before the given position, as that position is after this list");
        let result =
            Self::shallow_before(this.clone(), position).unwrap();
        this.borrow_mut().deflate(result.index, amount);
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub <= sub.borrow().last_position() {
                Self::deflate_before(sub, position_in_sub, amount);
            }
        }
    }

    pub fn inflate_unchecked(&mut self, index: usize, amount: S) {
        // ╭───────────────────────────────╮
        // ├───────────────╮               ├───────────────╮
        // ├───────╮       ├───────╮       ├───────╮       │
        // ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮
        // ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵
        // 00000   00010   00100   00110   01000   01010   01100   01110   10000
        //     00001   00011   00101   00111   01001   01011   01101   01111
        //
        // for degree in 0..self.depth {
        // TODO invent a metric that measures the "relative depth" above a link, i. e. how many
        //  links there *could* be above it, and use it in deflate_unchecked too
        for degree in 0.. {
            if index >> degree & 1 == 0 {
                if !self.link_index_is_in_bounds(link_index(index, degree)) {
                    break;
                }
                self.links[link_index(index, degree)] += amount;
            }
        }
        self.length += amount
    }

    pub fn inflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero(), "Cannot inflate by negative amount, explicitly deflate for that");
        self.inflate_unchecked(index, amount)
    }

    pub fn deflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0.. {
            if index >> degree & 1 == 0 {
                if !self.link_index_is_in_bounds(link_index(index, degree)) {
                    break;
                }
                self.links[link_index(index, degree)] -= amount;
            }
        }
        self.length -= amount
    }

    pub fn deflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero(), "Cannot deflate by negative amount, explicitly inflate for that");
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 {
                assert!(self.link(link_index(index, degree)) >= amount,
                        "Deflating at this index would deflate a link below zero");
            }
        }
        self.deflate_unchecked(index, amount)
    }

    pub fn inflate_after_offset(&mut self, amount: S) {
        if !self.links.is_empty() {
            self.inflate(0, amount);
            if let Some(sub) = self.sub(0) {
                sub.borrow_mut().offset += amount;
            }
        }
    }
}