use std::ops::Neg;
use num_traits::zero;

use crate::{SpacedListSkeleton, Spacing, Todo};

pub trait SpacedList<S: Spacing>: Default {
    fn skeleton(&self) -> &SpacedListSkeleton<S, Self>;

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self>;

    fn size(&self) -> usize;

    fn size_mut(&mut self) -> &mut usize;

    fn deep_size(&self) -> usize;

    fn deep_size_mut(&mut self) -> &mut usize;

    fn length(&self) -> S {
        self.skeleton().length()
    }

    fn deep_length(&self) -> S;

    fn deep_length_mut(&mut self) -> &mut S;

    fn append_node(&mut self, distance: S) {
        let size = self.size();
        if size == self.skeleton().size() {
            self.skeleton_mut().grow();
        }
        self.skeleton_mut().inflate_at(size, distance);
        *self.size_mut() += 1;
        *self.deep_size_mut() += 1;
        *self.deep_length_mut() += distance;
    }

    fn insert_node(&mut self, position: S) {
        todo!()
    }

    fn inflate_after(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    fn inflate_before(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    fn deflate_after(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    fn deflate_before(&mut self, node_index: Todo, amount: S) {
        todo!()
    }

    // All possible queries:
    // - first
    // - last before
    // - first at or last before
    // - last at or last before
    // - first at
    // - last at
    // - first at or first after
    // - last at or first after
    // - first after
    // - last
    //
    // TODO long term implement all of these

    fn node_before(&self, position: S) -> Todo {
        todo!()
    }

    fn node_at_or_before(&self, position: S) -> Todo {
        todo!()
    }

    fn node_at(&self, position: S) -> Todo {
        todo!()
    }

    fn node_at_or_after(&self, position: S) -> Todo {
        todo!()
    }

    fn node_after(&self, position: S) -> Todo {
        todo!()
    }
}