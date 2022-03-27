use std::marker::PhantomData;

use num_traits::zero;

use crate::{SpacedList, Spacing};

pub struct SpacedListSkeleton<S: Spacing, Sub: SpacedList<S>> {
    link_lengths: Vec<S>,
    sublists: Vec<Option<Sub>>,
    size: usize,
    depth: usize,
    length: S,
}

impl<S: Spacing, Sub: SpacedList<S>> Default for SpacedListSkeleton<S, Sub> {
	fn default() -> Self {
		Self {
			link_lengths: vec![],
			sublists: vec![],
			size: 0,
			depth: 0,
			length: zero()
		}
	}
}