use paste::paste;

use crate::{Position, SpacedList, Spacing};
use crate::positions::shallow::ShallowPosition;
use crate::traversal::*;

macro_rules! range_traversal_methods {
    {$($pos:ident: $cmp:tt)+} => {
        paste! {
            $(fn [<range_starting_ $pos>]<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
                where S: 'a {
                traverse!((range start); deep; self; $cmp target)
            })+
            $(fn [<range_ending_ $pos>]<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
                where S: 'a {
                traverse!((range end); deep; self; $cmp target)
            })+
        }
    };
}

#[allow(unused)]
pub trait RangeSpacedList<S: Spacing>: SpacedList<S> {
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

    /// 4-6 7-8
    /// insert_range(2, 1)
    /// 2-3 7-8
    ///    \1-3
    ///
    /// 4-6 7-8
    /// 2-4 5-6
    /// 2-6 7-8
    ///
    ///
    ///   ________d-a________
    ///   _____c-a_____
    ///   __b-a__
    /// ( a ) ( b ) ( c ) ( d )
    /// insert(( e ), (f-e))
    ///   ________d-e________ = (d-a) + (a-e)
    ///   _____c-e_____       = (c-a) + (a-e)
    ///   __f-e__             = (b-a) - (b-a) + (f-e)
    /// ( e ) ( f ) ( c ) ( d )
    ///            \(a-f) (b-f)
    fn insert_range<'a>(&'a mut self, position: S, span: S) -> Position<'a, S, Self> where S: 'a {
        if position + span < self.offset() {
            let offset = self.offset();
            let previous_span = self.link_length_at(0);
            *self.link_size_deep_mut() += 2;
            *self.node_size_deep_mut() += 2;
            *self.offset_mut() = position;
            if self.link_size() > 1 {
                self.inflate_after(position, offset - position);
            }
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
        sublist.insert_range(position - node_position, span)
    }

    range_traversal_methods! {
        before: <
        at_or_before: <=
        at: ==
        at_or_after: >=
        after: >
    }
}

pub(crate) mod hollow_range_spaced_list;

pub(crate) mod filled_range_spaced_list;
