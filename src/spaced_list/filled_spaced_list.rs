use num_traits::zero;
use paste::paste;

use crate::{Position, SpacedList, CrateSpacedList, Spacing};

spaced_list!(Filled);

macro_rules! element_traversal_methods {
    (@$pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_ $pos>](&self, target: S) -> Option<&T> {
                Some(self.element(self.[<node_ $pos>](target)?))
            }
        }
    };
    (@mut $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<element_ $pos _mut>](&mut self, target: S) -> Option<&mut T> {
                todo!() // TODO(mut)
            }
        }
    };
    () => {
        for_all_traversals!(element_traversal_methods @);
        for_all_traversals!(element_traversal_methods @mut);
    }
}

#[allow(unused)]
impl<S: Spacing, T> FilledSpacedList<S, T> {
    fn element_index(index: usize) -> usize {
        index
    }

    pub fn append_element(&mut self, distance: S, element: T) -> Position<S, Self> {
        self.elements.push(element);
        <Self as CrateSpacedList<S>>::append_node(self, distance)
    }

    pub fn insert_element(&mut self, position: S, element: T) -> Position<S, Self> {
        todo!()
    }

    element_traversal_methods!();
}
