use std::default::default;
use crate::{SpacedList, Todo};
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct HollowSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>
}

impl<S: Spacing> HollowSpacedList<S> {
    pub fn new() -> Self {
        default()
    }

    fn append_node(&mut self, distance: S) {
        todo!()
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

impl<S: Spacing> Default for HollowSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: default()
        }
    }
}

impl<S: Spacing> SpacedList<S> for HollowSpacedList<S> {

}