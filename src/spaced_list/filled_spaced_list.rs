use std::mem;
use num_traits::zero;
use paste::paste;

use crate::{Position, SpacedList, CrateSpacedList, Spacing};
use crate::positions::shallow::ShallowPosition;
use crate::traversal::*;

spaced_list!(Filled);

macro_rules! element_traversal_methods {
    (@$pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_ $pos>](&self, target: S) -> Option<&T> {
                Some(self.element(self.[<node_ $pos>](target)?))
            }
        }
    };
    (@mut $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_ $pos _mut>](&mut self, target: S) -> Option<&mut T> {
                todo!() // TODO(mut)
            }
        }
    };
    () => {
        for_all_traversals!(element_traversal_methods @);
        for_all_traversals!(element_traversal_methods @mut);
    }
}

#[allow(unused)]
impl<S: Spacing, T> FilledSpacedList<S, T> {
    fn element_index(index: usize) -> usize {
        index
    }

    pub fn append_element(&mut self, distance: S, element: T) -> Position<S, Self> {
        self.elements.push(element);
        <Self as CrateSpacedList<S>>::append_node(self, distance)
    }

    pub fn insert_element(&mut self, position: S, element: T) -> Position<S, Self> {
        if position < self.offset() {
            let offset = self.offset();
            let previous_element = mem::replace(&mut self.elements[0], element);
            *self.offset_mut() = position;
            if self.link_size() > 0 {
                self.inflate_after(self.offset(), offset - position);
            }
            self.insert_element(offset, previous_element);
            return Position::new(vec![], self, 0, position);
        }
        if position >= self.last_position() {
            return self.append_element(position - self.last_position(), element);
        }
        let ShallowPosition { index, position: node_position, .. } =
            traverse!(node; shallow; &*self; <= position).unwrap();
        *self.link_size_deep_mut() += 1;
        *self.node_size_deep_mut() += 1;
        let sublist = self.get_or_add_sublist_at_mut(index);
        sublist.insert_element(position - node_position, element)
    }

    element_traversal_methods!();
}
