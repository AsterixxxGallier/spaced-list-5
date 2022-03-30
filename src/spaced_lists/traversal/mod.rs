use crate::{Spacing, SpacedList};

pub(crate) mod node;

pub(crate) mod range;

pub struct Position<'a, S: Spacing, List: SpacedList<S>> {
    lists: Vec<&'a List>,
    pub index: usize,
    pub position: S,
    link_index: usize,
    offset: usize,
    mask: usize,
}