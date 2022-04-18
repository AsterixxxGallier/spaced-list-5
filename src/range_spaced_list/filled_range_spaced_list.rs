use std::mem;
use num_traits::zero;
use paste::paste;

use crate::{Position, RangeSpacedList, CrateSpacedList, Spacing, SpacedList};
use crate::iteration::range::RangeIter;
use crate::positions::shallow::ShallowPosition;
use crate::traversal::*;

spaced_list!(Filled Range);

macro_rules! element_of_range_traversal_methods {
    (@$bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_of_range_ $bound ing_ $pos>](&self, target: S) -> Option<&T> {
                Some(self.element(&self.[<range_ $bound ing_ $pos>](target)?))
            }
        }
    };
    (@mut $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_of_range_ $bound ing_ $pos _mut>](&mut self, target: S) -> Option<&mut T> {
                todo!() // TODO(mut)
            }
        }
    };
    () => {
        for_all_traversals!(element_of_range_traversal_methods @start);
        for_all_traversals!(element_of_range_traversal_methods @end);
        for_all_traversals!(element_of_range_traversal_methods @mut start);
        for_all_traversals!(element_of_range_traversal_methods @mut end);
    }
}

#[allow(unused)]
impl<S: Spacing, T> FilledRangeSpacedList<S, T> {
    fn element_index(index: usize) -> usize {
        index / 2
    }

    pub fn append_range(&mut self, distance: S, span: S, element: T) -> Position<S, Self> {
        self.elements.push(element);
        <Self as RangeSpacedList<S>>::append_range(self, distance, span)
    }

    pub fn insert_range(&mut self, position: S, span: S, element: T) -> Position<S, Self> {
        if position + span < self.offset() {
            let offset = self.offset();
            let previous_span = self.link_length_at(0);
            let previous_element = mem::replace(&mut self.elements[0], element);
            *self.link_size_deep_mut() += 2;
            *self.node_size_deep_mut() += 2;
            *self.offset_mut() = position;
            if self.link_size() > 1 {
                self.inflate_after(position, offset - position);
            }
            *self.link_length_at_mut(0) = span;
            self.insert_range( offset, previous_span, previous_element);
            return Position::new(vec![], self, 0, position);
        }
        if position >= self.last_position() {
            return self.append_range(
                position - self.last_position(), span, element);
        }
        let ShallowPosition { index, position: node_position, .. } =
            traverse!(node; shallow; &*self; <= position).unwrap();
        assert_eq!(index & 1, 1, "Cannot insert range inside of another range");
        *self.link_size_deep_mut() += 2;
        *self.node_size_deep_mut() += 2;
        let link_length = self.link_length_at_node(index);
        let sublist = self.get_or_add_sublist_at_mut(index);
        assert!(position - node_position + span < link_length,
                "There is not enough space for this range between the neighbouring ranges");
        sublist.insert_range(position - node_position, span, element)
    }

    element_of_range_traversal_methods!();

    pub fn iter(&self) -> RangeIter<S, Self> {
        RangeIter::new(self)
    }
}
