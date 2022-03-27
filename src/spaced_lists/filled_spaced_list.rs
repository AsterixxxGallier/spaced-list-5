use crate::{SpacedList, Todo};
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct FilledSpacedList<S: Spacing, T> {
    skeleton: SpacedListSkeleton<S, Self>,
    elements: Vec<T>
}

impl<S: Spacing, T> FilledSpacedList<S, T> {
    fn element(&self, index: Todo) -> &T {
        todo!()
    }

    fn element_mut(&mut self, index: Todo) -> &mut T {
        todo!()
    }

    fn append_element(&mut self, distance: S, element: T) {
        todo!()
    }

    fn insert_element(&mut self, position: S, element: T) {
        todo!()
    }

    fn element_before(&self, position: S) -> &T {
        self.element(self.node_before(position))
    }

    fn element_at_or_before(&self, position: S) -> &T {
        self.element(self.node_at_or_before(position))
    }

    fn element_at(&self, position: S) -> &T {
        self.element(self.node_at(position))
    }

    fn element_at_or_after(&self, position: S) -> &T {
        self.element(self.node_at_or_after(position))
    }

    fn element_after(&self, position: S) -> &T {
        self.element(self.node_after(position))
    }

    fn element_before_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_before(position))
    }

    fn element_at_or_before_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_at_or_before(position))
    }

    fn element_at_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_at(position))
    }

    fn element_at_or_after_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_at_or_after(position))
    }

    fn element_after_mut(&mut self, position: S) -> &mut T {
        self.element_mut(self.node_after(position))
    }
}

impl<S: Spacing, T> SpacedList<S> for FilledSpacedList<S, T> {

}