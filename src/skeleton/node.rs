use std::cell::{Ref, RefCell, RefMut};
use std::mem;
use std::rc::Rc;

use crate::skeleton::{link_index, Node, Skeleton, Spacing};

impl<S: Spacing, T> Skeleton<Node, S, T> {
    pub(crate) fn push(&mut self, distance: S, element: T) {
        if self.elements.is_empty() {
            self.offset = distance;
            self.elements.push(element);
            return;
        }
        let index = self.push_link();
        self.inflate(index, distance);
        self.elements.push(element);
    }

    pub(crate) fn insert(&mut self, position: S, element: T) {
        if self.elements.is_empty() {
            return self.push(position, element);
        }
        if position < self.offset {
            let previous_first_position = self.offset;
            let previous_first_element = mem::replace(&mut self.elements[0], element);
            let inflation_amount = previous_first_position - position;
            if !self.links.is_empty() {
                self.inflate(0, inflation_amount);
                if let Some(sub) = self.sub(0) {
                    sub.borrow_mut().offset += inflation_amount;
                }
            }
            self.offset = position;
            return self.insert(previous_first_position, previous_first_element);
        }
        if position >= self.last_position() {
            return self.push(position - self.last_position(), element);
        }
        todo!("Traverse this skeleton and insert into sublist")
    }

    fn before(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        if this.borrow().elements.is_empty() || target <= this.borrow().offset {
            None
        } else if this.borrow().links.is_empty() {
            if this.borrow().offset < target {
                Some(Position::new(this.clone(), 0, this.borrow().offset))
            } else {
                None
            }
        } else {
            let mut skeleton = this;
            let mut degree = skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = skeleton.borrow().offset;
            loop {
                let link_index = link_index(index, degree);
                if !skeleton.borrow().link_index_is_in_bounds(link_index) {
                    if degree == 0 {
                        break;
                    }
                    degree -= 1;
                    continue;
                }

                let next_position = position + skeleton.borrow().links[link_index];
                if next_position < target {
                    position = next_position;
                    index += 1 << degree;
                }

                if degree > 0 {
                    degree -= 1;
                } else {
                    if let Some(sub) = skeleton.clone().borrow().sub(index) {
                        let next_position = position + sub.borrow().offset;
                        if next_position < target {
                            degree = sub.borrow().depth.saturating_sub(1);
                            index = 0;
                            position = next_position;
                            skeleton = sub;
                            continue;
                        }
                    }
                    break;
                }
            }
            Some(Position::new(skeleton, index, position))
        }
    }
}

pub struct Position<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Node, S, T>>>,
    index: usize,
    position: S,
}

impl<S: Spacing, T> Clone for Position<S, T> {
    fn clone(&self) -> Self {
        Self {
            skeleton: self.skeleton.clone(),
            index: self.index,
            position: self.position,
        }
    }
}

impl<S: Spacing, T> Position<S, T> {
    pub(crate) fn new(skeleton: Rc<RefCell<Skeleton<Node, S, T>>>, index: usize, position: S) -> Self {
        Self {
            skeleton,
            index,
            position,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }

    pub fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index])
    }

    pub fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index])
    }
}