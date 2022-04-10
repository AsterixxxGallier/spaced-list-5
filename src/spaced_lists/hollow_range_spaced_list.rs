use std::default::default;

use paste::paste;

use crate::{SpacedList, Skeleton, Spacing, Position, Iter};
use crate::spaced_lists::positions::shallow::ShallowPosition;
use crate::spaced_lists::traversal::*;

spaced_list!(Hollow Range);

// TODO add a RangeSpacedList trait and move the method implementations below there
#[allow(unused)]
impl<S: Spacing> HollowRangeSpacedList<S> {
    pub fn append_range(&mut self, distance: S, span: S) -> Position<S, Self> {
        assert_eq!(self.skeleton().node_size() & 1, 0);
        let link_size = self.skeleton().link_size();
        let node_size = self.skeleton().node_size();
        if node_size == 0 {
            *self.skeleton_mut().node_size_mut() += 2;
            *self.skeleton_mut().node_size_deep_mut() += 2;
            *self.skeleton_mut().link_size_mut() += 1;
            *self.skeleton_mut().link_size_deep_mut() += 1;
            *self.skeleton_mut().offset_mut() = distance;
            self.skeleton_mut().grow();
            self.skeleton_mut().inflate_at(0, span);
            return Position::new(vec![], self, 0, distance);
        }
        while node_size + 1 >= self.skeleton().node_capacity() {
            self.skeleton_mut().grow();
        }
        *self.skeleton_mut().link_size_mut() += 2;
        *self.skeleton_mut().link_size_deep_mut() += 2;
        *self.skeleton_mut().node_size_mut() += 2;
        *self.skeleton_mut().node_size_deep_mut() += 2;
        self.skeleton_mut().inflate_at(link_size, distance);
        let position = self.skeleton().length();
        self.skeleton_mut().inflate_at(link_size + 1, span);
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
    pub fn insert_range(&mut self, position: S, span: S) -> Position<S, Self> {
        if position + span < self.skeleton().offset() {
            let offset = self.skeleton().offset();
            let previous_span = self.skeleton().link_length_at(0);
            self.print_positions();
            // 5 8 10 11
            *self.skeleton_mut().offset_mut() = position;
            self.print_positions();
            // 1 4 6 7
            // 1 3 6 7
            self.skeleton_mut().inflate_at(1, offset - position);
            self.print_positions();
            // 1 4 10 11
            *self.skeleton_mut().link_length_at_mut(0) = span;
            self.print_positions();
            // 1 4 10 11
            self.insert_range(offset, previous_span);
            self.print_positions();
            return Position::new(vec![], self, 0, position);
        }
        // TODO add a last_position() accessor to Skeleton and use it wherever possible
        if position >= self.skeleton().offset() + self.skeleton().length() {
            return self.append_range(
                position - self.skeleton().offset() - self.skeleton.length(), span);
        }
        let ShallowPosition { index, position: node_position, .. } =
            traverse!(shallow; &*self; <= position).unwrap();
        assert_eq!(index & 1, 1, "Cannot insert range inside of another range");
        *self.skeleton_mut().link_size_deep_mut() += 2;
        *self.skeleton_mut().node_size_deep_mut() += 2;
        let sublist = self.skeleton_mut().get_or_add_sublist_at_mut(index);
        sublist.insert_range(position - node_position, span)
    }

    pub fn print_positions(&self) {
        for pos in self.iter() {
            println!("{}", pos.position());
        }
        println!();
    }

    delegates! {
        iter(&self) -> Iter<S, Self>;

        inflate_after(&mut self, position: S, amount: S);
        inflate_before(&mut self, position: S, amount: S);
        deflate_after(&mut self, position: S, amount: S);
        deflate_before(&mut self, position: S, amount: S);
    }

    pub fn range_starting_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_at_or_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_at(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_at_or_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_starting_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_at_or_before(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_at(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_at_or_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }

    pub fn range_ending_after(&self, position: S) -> Position<S, Self> {
        todo!()
    }
}
