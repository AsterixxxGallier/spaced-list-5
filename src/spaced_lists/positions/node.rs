use crate::{Spacing, SpacedList};

pub struct Position<'list, S: Spacing, List: SpacedList<S>> {
    super_lists: Vec<&'list List>,
    list: &'list List,
    index: usize,
    position: S,
}

impl<'list, S: Spacing, List: SpacedList<S>> Position<'list, S, List> {
    pub(crate) fn new(
        super_lists: Vec<&'list List>,
        list: &'list List,
        index: usize,
        position: S,
    ) -> Self {
        Position {
            super_lists,
            list,
            index,
            position,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }
}