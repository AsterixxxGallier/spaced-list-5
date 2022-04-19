use crate::{Spacing, SpacedList};

pub struct Position<'list, S: Spacing, List: SpacedList<S>> {
    super_lists: Vec<&'list List>,
    list: &'list List,
    index: usize,
    position: S,
}

impl<'list, S: Spacing, List: SpacedList<S>> Clone for Position<'list, S, List> {
    fn clone(&self) -> Self {
        Self {
            super_lists: self.super_lists.clone(),
            list: self.list,
            index: self.index,
            position: self.position
        }
    }
}

// TODO implement Copy for Position by storing a reference to the super list along with the index in it

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

    accessors! {
        pub(crate) ref super_lists: Vec<&'list List>;
        pub(crate) list: &'list List;
        pub(crate) index: usize;
        pub position: S;
    }
}