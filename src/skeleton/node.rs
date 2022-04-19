use std::mem;
use crate::skeleton::{Spacing, Skeleton, Node};

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
                if let Some(sub) = self.subs.get_mut(0) {
                    sub.offset += inflation_amount;
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
}