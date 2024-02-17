use std::cell::RefCell;
use std::rc::Rc;

use num_traits::zero;

use crate::skeleton::{link_index, relative_depth, Skeleton};
use crate::Spacing;

#[derive(Debug)]
pub enum FlateError {
    /// "Cannot inflate/deflate after the given position, as that position is at or after the end of this list"
    PositionAtOrAfterList,
    /// "Cannot inflate/deflate before the given position, as that position is after the end of this list"
    PositionAfterList,
    /// "Cannot inflate/deflate by negative amount, explicitly deflate/inflate for that"
    AmountNegative,
    /// "Deflating at this index would deflate a link below zero"
    DeflationBelowZero,
}

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    pub fn try_inflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), FlateError> {
        if position < this.borrow().offset {
            this.borrow_mut().offset += amount;
            return Ok(());
        }
        if position >= this.borrow().last_position() {
            return Err(FlateError::PositionAtOrAfterList);
        }
        let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
        this.borrow_mut().try_inflate(result.index, amount)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub < sub.borrow().last_position() {
                Self::try_inflate_after(sub, position_in_sub, amount)?;
            }
        }
        Ok(())
    }

    pub fn try_inflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), FlateError> {
        if position <= this.borrow().offset {
            this.borrow_mut().offset += amount;
            return Ok(());
        }
        if position > this.borrow().last_position() {
            return Err(FlateError::PositionAfterList);
        }
        let result = Self::shallow_before(this.clone(), position).unwrap();
        this.borrow_mut().try_inflate(result.index, amount)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub <= sub.borrow().last_position() {
                Self::try_inflate_before(sub, position_in_sub, amount)?;
            }
        }
        Ok(())
    }

    pub fn try_deflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), FlateError> {
        if position < this.borrow().offset {
            this.borrow_mut().offset -= amount;
            return Ok(());
        }
        if position >= this.borrow().last_position() {
            return Err(FlateError::PositionAtOrAfterList);
        }
        let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
        this.borrow_mut().try_deflate(result.index, amount)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub < sub.borrow().last_position() {
                Self::try_deflate_after(sub, position_in_sub, amount)?;
            }
        }
        Ok(())
    }

    pub fn try_deflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), FlateError> {
        if position <= this.borrow().offset {
            this.borrow_mut().offset -= amount;
            return Ok(());
        }
        if position > this.borrow().last_position() {
            return Err(FlateError::PositionAfterList);
        }
        let result = Self::shallow_before(this.clone(), position).unwrap();
        this.borrow_mut().try_deflate(result.index, amount)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub <= sub.borrow().last_position() {
                Self::try_deflate_before(sub, position_in_sub, amount)?;
            }
        }
        Ok(())
    }

    pub fn inflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..relative_depth(index, self.links.len()) {
            if index >> degree & 1 == 0 {
                self.links[link_index(index, degree)] += amount;
            }
        }
        self.length += amount;
    }

    pub fn try_inflate(&mut self, index: usize, amount: S) -> Result<(), FlateError> {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        if amount < zero() {
            return Err(FlateError::AmountNegative);
        }
        self.inflate_unchecked(index, amount);
        Ok(())
    }

    pub fn deflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..relative_depth(index, self.links.len()) {
            if index >> degree & 1 == 0 {
                self.links[link_index(index, degree)] -= amount;
            }
        }
        self.length -= amount;
    }

    pub fn try_deflate(&mut self, index: usize, amount: S) -> Result<(), FlateError> {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        if amount < zero() {
            return Err(FlateError::AmountNegative);
        }
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 && self.link(link_index(index, degree)) < amount {
                return Err(FlateError::DeflationBelowZero);
            }
        }
        self.deflate_unchecked(index, amount);
        Ok(())
    }

    pub fn try_inflate_after_index(&mut self, index: usize, amount: S) -> Result<(), FlateError> {
        if amount < zero() {
            return Err(FlateError::AmountNegative);
        }
        if self.link_index_is_in_bounds(index) {
            self.try_inflate(index, amount)?;
            if let Some(sub) = self.sub(index) {
                sub.borrow_mut().offset += amount;
            }
        }
        Ok(())
    }

    pub fn try_deflate_after_index(&mut self, index: usize, amount: S) -> Result<(), FlateError> {
        if amount < zero() {
            return Err(FlateError::AmountNegative);
        }
        if self.link_index_is_in_bounds(index) {
            self.try_deflate(index, amount)?;
            if let Some(sub) = self.sub(index) {
                sub.borrow_mut().offset -= amount;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use std::collections::HashMap;

    use crate::skeleton::{link_index, relative_depth};

    #[test]
    fn test() {
        // let mut approaches = HashMap::new();
        // approaches.insert("xor size", |size: usize, index: usize|
        //     (usize::BITS - (size ^ index).leading_zeros()) as usize);
        for size in 0usize..16 {
            for index in 0usize..size {
                let depth =
                    (0..)
                        .take_while(|&degree| link_index(index, degree) < size)
                        .count();
                assert_eq!(relative_depth(index, size), depth);
                // println!("{:04b}/{:04b}=>{}", index, size, depth);
                // let mut to_remove = vec![];
                // for (&key, &value) in &approaches {
                //     if value(size, index) != depth {
                //         to_remove.push(key);
                //     }
                // }
                // for key in to_remove {
                //     approaches.remove(key);
                // }
            }
        }
        // println!("{:?}", approaches.keys());
    }
}