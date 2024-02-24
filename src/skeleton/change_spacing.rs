use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use num_traits::zero;

use crate::{display_unwrap, Skeleton, Spacing};
use crate::skeleton::{get_link_index, relative_depth};
use crate::spaced_lists::spacing_error::SpacingError;

// NOTE FOR DUMMIES (LIKE ME): there are separate increase and decrease functions because S might be a non-negative type
//  like usize, so increasing by a negative value is impossible, so we need extra decrease functions

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    pub fn increase_spacing_after(this: Rc<RefCell<Self>>, position: S, change: S) {
        display_unwrap!(Self::try_increase_spacing_after(this, position, change));
    }

    pub fn increase_spacing_before(this: Rc<RefCell<Self>>, position: S, change: S) {
        display_unwrap!(Self::try_increase_spacing_before(this, position, change));
    }

    pub fn decrease_spacing_after(this: Rc<RefCell<Self>>, position: S, change: S) {
        display_unwrap!(Skeleton::try_decrease_spacing_after(this, position, change));
    }

    pub fn decrease_spacing_before(this: Rc<RefCell<Self>>, position: S, change: S) {
        display_unwrap!(Skeleton::try_decrease_spacing_before(this, position, change));
    }


    pub fn try_increase_spacing_after(this: Rc<RefCell<Self>>, position: S, change: S) -> Result<(), SpacingError<S>> {
        if position >= this.borrow().last_position() {
            return Err(SpacingError::PositionAtOrAfterList { position });
        }
        if change == zero() {
            return Ok(());
        }
        if position < this.borrow().offset {
            this.borrow_mut().offset += change;
            return Ok(());
        }
        let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
        this.borrow_mut().try_increase_spacing(result.index, position, change)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub < sub.borrow().last_position() {
                Self::try_increase_spacing_after(sub, position_in_sub, change)?;
            }
        }
        Ok(())
    }

    pub fn try_increase_spacing_before(this: Rc<RefCell<Self>>, position: S, change: S) -> Result<(), SpacingError<S>> {
        if position > this.borrow().last_position() {
            return Err(SpacingError::PositionAfterList { position });
        }
        if change == zero() {
            return Ok(());
        }
        if position <= this.borrow().offset {
            this.borrow_mut().offset += change;
            return Ok(());
        }
        let result = Self::shallow_before(this.clone(), position).unwrap();
        this.borrow_mut().try_increase_spacing(result.index, position, change)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub <= sub.borrow().last_position() {
                Self::try_increase_spacing_before(sub, position_in_sub, change)?;
            }
        }
        Ok(())
    }

    pub fn try_decrease_spacing_after(this: Rc<RefCell<Self>>, position: S, change: S) -> Result<(), SpacingError<S>> {
        if position >= this.borrow().last_position() {
            return Err(SpacingError::PositionAtOrAfterList { position });
        }
        if change == zero() {
            return Ok(());
        }
        if position < this.borrow().offset {
            this.borrow_mut().offset -= change;
            return Ok(());
        }
        let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
        this.borrow_mut().try_decrease_spacing(result.index, position, change)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub < sub.borrow().last_position() {
                Self::try_decrease_spacing_after(sub, position_in_sub, change)?;
            }
        }
        Ok(())
    }

    pub fn try_decrease_spacing_before(this: Rc<RefCell<Self>>, position: S, change: S) -> Result<(), SpacingError<S>> {
        if position > this.borrow().last_position() {
            return Err(SpacingError::PositionAfterList { position });
        }
        if change == zero() {
            return Ok(());
        }
        if position <= this.borrow().offset {
            this.borrow_mut().offset -= change;
            return Ok(());
        }
        let result = Self::shallow_before(this.clone(), position).unwrap();
        this.borrow_mut().try_decrease_spacing(result.index, position, change)?;
        if let Some(sub) = this.borrow_mut().sub(result.index) {
            let position_in_sub = position - result.position;
            if position_in_sub <= sub.borrow().last_position() {
                Self::try_decrease_spacing_before(sub, position_in_sub, change)?;
            }
        }
        Ok(())
    }


    pub(super) fn increase_spacing_after_index(&mut self, index: usize, change: S) {
        if self.link_index_is_in_bounds(index) {
            self.increase_spacing(index, change);
            if let Some(sub) = self.sub(index) {
                sub.borrow_mut().offset += change;
            }
        }
    }

    pub(super) fn increase_spacing(&mut self, index: usize, change: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        match change.cmp(&zero()) {
            Ordering::Less => {
                self.decrease_spacing(index, zero::<S>() - change);
            }
            Ordering::Equal => {},
            Ordering::Greater => {
                self.increase_spacing_unchecked(index, change);
            }
        }
    }

    pub(super) fn try_increase_spacing(&mut self, index: usize, position: S, change: S) -> Result<(), SpacingError<S>> {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        match change.cmp(&zero()) {
            Ordering::Less => {
                self.try_decrease_spacing(index, position, zero::<S>() - change)
            },
            Ordering::Equal => {
                Ok(())
            },
            Ordering::Greater => {
                self.increase_spacing_unchecked(index, change);
                Ok(())
            }
        }
    }

    pub(super) fn increase_spacing_unchecked(&mut self, index: usize, change: S) {
        for degree in 0..relative_depth(index, self.links.len()) {
            if index >> degree & 1 == 0 {
                self.links[get_link_index(index, degree)] += change;
            }
        }
        self.length += change;
    }


    pub(super) fn decrease_spacing_after_index(&mut self, index: usize, change: S) {
        if self.link_index_is_in_bounds(index) {
            self.decrease_spacing(index, change);
            if let Some(sub) = self.sub(index) {
                sub.borrow_mut().offset -= change;
            }
        }
    }

    pub(super) fn decrease_spacing(&mut self, index: usize, change: S) {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        match change.cmp(&zero()) {
            Ordering::Less => {
                self.increase_spacing(index, zero::<S>() - change);
            }
            Ordering::Equal => {},
            Ordering::Greater => {
                for degree in 0..self.depth {
                    assert!(!(index >> degree & 1 == 0 && self.link(get_link_index(index, degree)) < change));
                }
                self.decrease_spacing_unchecked(index, change);
            }
        }
    }

    pub(super) fn try_decrease_spacing(&mut self, index: usize, position: S, change: S) -> Result<(), SpacingError<S>> {
        assert!(self.link_index_is_in_bounds(index), "Index not in bounds");
        match change.cmp(&zero()) {
            Ordering::Less => {
                self.try_increase_spacing(index, position, zero::<S>() - change)
            }
            Ordering::Equal => {
                Ok(())
            },
            Ordering::Greater => {
                for degree in 0..self.depth {
                    if index >> degree & 1 == 0 && self.link(get_link_index(index, degree)) < change {
                        return Err(SpacingError::SpacingNotLargeEnough {
                            position,
                            change,
                            spacing: self.link(index)
                        });
                    }
                }
                self.decrease_spacing_unchecked(index, change);
                Ok(())
            }
        }
    }

    pub(super) fn decrease_spacing_unchecked(&mut self, index: usize, change: S) {
        for degree in 0..relative_depth(index, self.links.len()) {
            if index >> degree & 1 == 0 {
                self.links[get_link_index(index, degree)] -= change;
            }
        }
        self.length -= change;
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