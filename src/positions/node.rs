use crate::{Spacing, SpacedList};

// FIXME: This is a public-facing struct, but List, one of its type parameters, has a private type
//  this means that there is no way to specify this struct without specifying exactly which specific
//  SpacedList implementor is meant
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

    accessors! {
        pub(crate) ref super_lists: Vec<&'list List>;
        pub(crate) list: &'list List;
        pub(crate) index: usize;
        pub position: S;
    }
}