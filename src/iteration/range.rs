use crate::{Spacing, Iter, Position, RangeSpacedList};

pub struct RangeIter<'list, S: Spacing, List: RangeSpacedList<S>> {
    iter: Iter<'list, S, List>
}

impl<'list, S: Spacing, List: RangeSpacedList<S>> Iterator for RangeIter<'list, S, List> {
    type Item = (Position<'list, S, List>, Position<'list, S, List>);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.iter.next()?, self.iter.next().unwrap()))
    }
}