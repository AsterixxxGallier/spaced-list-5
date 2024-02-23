use std::cell::RefCell;
use std::rc::Rc;

use num_traits::zero;

use crate::{Spacing, Skeleton, SpacingError, SpacingOperation};
use crate::skeleton::{get_link_index, relative_depth};

macro_rules! display_unwrap {
    ($arg:expr) => {
        match $arg {
            Err(error) => panic!("{}", error),
            Ok(value) => value
        }
    };
}

// TODO refactor this and all increase/decrease_spacing functions to
//  - do nothing if amount = 0
//  - check that all spacings are large enough to be changed by amount if amount < 0
//  - "inflate" by amount

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    pub fn inflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) {
        display_unwrap!(Skeleton::try_inflate_after(this, position, amount));
    }

    pub fn inflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) {
        display_unwrap!(Skeleton::try_inflate_before(this, position, amount));
    }

    pub fn deflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) {
        display_unwrap!(Skeleton::try_deflate_after(this, position, amount));
    }

    pub fn deflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) {
        display_unwrap!(Skeleton::try_deflate_before(this, position, amount));
    }


    pub fn try_inflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), SpacingError<S>> {
        if position < this.borrow().offset {
            this.borrow_mut().offset += amount;
            return Ok(());
        }
        if position >= this.borrow().last_position() {
            return Err(SpacingError::PositionAtOrAfterList {
                operation: SpacingOperation::Increase,
                position
            });
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

    pub fn try_inflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), SpacingError<S>> {
        if position <= this.borrow().offset {
            this.borrow_mut().offset += amount;
            return Ok(());
        }
        if position > this.borrow().last_position() {
            return Err(SpacingError::PositionAfterList {
                operation: SpacingOperation::Increase,
                position
            });
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

    pub fn try_deflate_after(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), SpacingError<S>> {
        if position < this.borrow().offset {
            this.borrow_mut().offset -= amount;
            return Ok(());
        }
        if position >= this.borrow().last_position() {
            return Err(SpacingError::PositionAtOrAfterList {
                operation: SpacingOperation::Decrease,
                position
            });
        }
        let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
        this.borrow_mut().try_deflate(result.index, position, amount)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub < sub.borrow().last_position() {
                Self::try_deflate_after(sub, position_in_sub, amount)?;
            }
        }
        Ok(())
    }

    pub fn try_deflate_before(this: Rc<RefCell<Self>>, position: S, amount: S) -> Result<(), SpacingError<S>> {
        if position <= this.borrow().offset {
            this.borrow_mut().offset -= amount;
            return Ok(());
        }
        if position > this.borrow().last_position() {
            return Err(SpacingError::PositionAfterList {
                operation: SpacingOperation::Decrease,
                position
            });
        }
        let result = Self::shallow_before(this.clone(), position).unwrap();
        this.borrow_mut().try_deflate(result.index, position, amount)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub <= sub.borrow().last_position() {
                Self::try_deflate_before(sub, position_in_sub, amount)?;
            }
        }
        Ok(())
    }


    pub(super) fn inflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..relative_depth(index, self.links.len()) {
            if index >> degree & 1 == 0 {
                self.links[get_link_index(index, degree)] += amount;
            }
        }
        self.length += amount;
    }

    pub(super) fn try_inflate(&mut self, index: usize, amount: S) -> Result<(), SpacingError<S>> {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        if amount < zero() {
            return Err(SpacingError::AmountNegative {
                operation: SpacingOperation::Increase,
                amount
            });
        }
        self.inflate_unchecked(index, amount);
        Ok(())
    }

    pub(super) fn inflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero());
        self.inflate_unchecked(index, amount);
    }

    pub(super) fn try_deflate(&mut self, index: usize, position: S, amount: S) -> Result<(), SpacingError<S>> {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        if amount < zero() {
            return Err(SpacingError::AmountNegative {
                operation: SpacingOperation::Decrease,
                amount
            });
        }
        for degree in 0..self.depth {
            if index >> degree & 1 == 0 && self.link(get_link_index(index, degree)) < amount {
                return Err(SpacingError::SpacingNotLargeEnough {
                    position,
                    amount,
                    spacing: self.link(index)
                });
            }
        }
        self.deflate_unchecked(index, amount);
        Ok(())
    }


    pub(super) fn deflate(&mut self, index: usize, amount: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        assert!(amount >= zero());
        for degree in 0..self.depth {
            assert!(!(index >> degree & 1 == 0 && self.link(get_link_index(index, degree)) < amount));
        }
        self.deflate_unchecked(index, amount);
    }

    pub(super) fn deflate_unchecked(&mut self, index: usize, amount: S) {
        for degree in 0..relative_depth(index, self.links.len()) {
            if index >> degree & 1 == 0 {
                self.links[get_link_index(index, degree)] -= amount;
            }
        }
        self.length -= amount;
    }


    pub(super) fn inflate_after_index(&mut self, index: usize, amount: S) {
        assert!(amount >= zero());
        if self.link_index_is_in_bounds(index) {
            self.inflate(index, amount);
            if let Some(sub) = self.sub(index) {
                sub.borrow_mut().offset += amount;
            }
        }
    }

    pub(super) fn deflate_after_index(&mut self, index: usize, amount: S) {
        assert!(amount >= zero());
        if self.link_index_is_in_bounds(index) {
            self.deflate(index, amount);
            if let Some(sub) = self.sub(index) {
                sub.borrow_mut().offset -= amount;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use std::collections::HashMap;

    use crate::skeleton::{get_link_index, relative_depth};

    #[test]
    fn test() {
        // let mut approaches = HashMap::new();
        // approaches.insert("xor size", |size: usize, index: usize|
        //     (usize::BITS - (size ^ index).leading_zeros()) as usize);
        for size in 0usize..16 {
            for index in 0usize..size {
                let depth =
                    (0..)
                        .take_while(|&degree| get_link_index(index, degree) < size)
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