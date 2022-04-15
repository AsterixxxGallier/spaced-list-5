use num_traits::zero;
use paste::paste;

use crate::{Position, RangeSpacedList, CrateSpacedList, Spacing, SpacedList};

spaced_list!(Filled Range);

macro_rules! element_of_range_traversal_methods {
    (@$bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_of_range_ $bound ing_ $pos>](&self, target: S) -> Option<&T>
                where Self: SpacedList<S> {
                Some(self.element(self.[<range_ $bound ing_ $pos>](target)?))
            }
        }
    };
    (@mut $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_of_range_ $bound ing_ $pos _mut>](&mut self, target: S) -> Option<&mut T>
                where Self: SpacedList<S> {
                todo!() // TODO
            }
        }
    };
    () => {
        for_all_traversals!(element_of_range_traversal_methods @start);
        for_all_traversals!(element_of_range_traversal_methods @end);
        for_all_traversals!(element_of_range_traversal_methods @mut start);
        for_all_traversals!(element_of_range_traversal_methods @mut end);
    }
}

#[allow(unused)]
impl<S: Spacing, T> FilledRangeSpacedList<S, T> {
    fn element_index(index: usize) -> usize {
        index / 2
    }

    pub fn append_range(&mut self, distance: S, span: S, element: T) -> Position<S, Self>
        where Self: SpacedList<S> {
        todo!()
    }

    pub fn insert_range(&mut self, position: S, span: S, element: T) -> Position<S, Self>
        where Self: SpacedList<S> {
        todo!()
    }

    element_of_range_traversal_methods!();
}
