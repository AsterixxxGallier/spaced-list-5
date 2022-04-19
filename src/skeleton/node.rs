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
}