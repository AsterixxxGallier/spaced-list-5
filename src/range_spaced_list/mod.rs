use paste::paste;

use crate::{Position, CrateSpacedList, Spacing, SpacedList};
use crate::positions::shallow::ShallowPosition;
use crate::traversal::*;

macro_rules! range_traversal_methods {
    (@$bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            fn [<range_ $bound ing_ $pos>]<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
                where S: 'a {
                traverse!((range $bound); deep; self; $cmp target)
            }
        }
    };
    () => {
        for_all_traversals!(range_traversal_methods @start);
        for_all_traversals!(range_traversal_methods @end);
    }
}

#[allow(unused)]
pub trait RangeSpacedList<S: Spacing>: CrateSpacedList<S> + SpacedList<S> {
    // TODO add try_ versions of the methods below

    fn append_range(&mut self, distance: S, span: S) -> Position<S, Self> {
        assert_eq!(self.node_size() & 1, 0);
        let link_size = self.link_size();
        let node_size = self.node_size();
        if node_size == 0 {
            *self.node_size_mut() += 2;
            *self.node_size_deep_mut() += 2;
            *self.link_size_mut() += 1;
            *self.link_size_deep_mut() += 1;
            *self.offset_mut() = distance;
            self.grow();
            self.inflate_at(0, span);
            return Position::new(vec![], self, 0, distance);
        }
        while node_size + 1 >= self.node_capacity() {
            self.grow();
        }
        *self.link_size_mut() += 2;
        *self.link_size_deep_mut() += 2;
        *self.node_size_mut() += 2;
        *self.node_size_deep_mut() += 2;
        self.inflate_at(link_size, distance);
        let position = self.length();
        self.inflate_at(link_size + 1, span);
        Position::new(vec![], self, link_size, position)
    }

    fn insert_range<'a>(&'a mut self, position: S, span: S) -> Position<'a, S, Self> where S: 'a {
        if self.node_size() == 0 {
            return self.append_range(position, span);
        }
        if position + span < self.offset() {
            let offset = self.offset();
            let previous_span = self.link_length_at(0);
            *self.link_size_deep_mut() += 2;
            *self.node_size_deep_mut() += 2;
            *self.offset_mut() = position;
            let amount = offset - position;
            if self.link_size() > 1 {
                self.inflate_at(0, amount);
                if let Some(sublist) = self.sublist_at_mut(0) {
                    *sublist.offset_mut() += amount;
                }
            };
            *self.link_length_at_mut(0) = span;
            self.insert_range(offset, previous_span);
            return Position::new(vec![], self, 0, position);
        }
        if position >= self.last_position() {
            return self.append_range(
                position - self.last_position(), span);
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
        let position_in_sublist = sublist.insert_range(position - node_position, span);
        // TODO avoid the clone here
        Position::new(
            position_in_sublist.super_lists().clone(),
            position_in_sublist.list(),
            position_in_sublist.index(),
            position_in_sublist.position() + node_position
        )
    }

    range_traversal_methods!();
}

pub(crate) mod hollow_range_spaced_list;

pub(crate) mod filled_range_spaced_list;
