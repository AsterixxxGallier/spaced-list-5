use std::marker::PhantomData;
use crate::{SpacedList, Spacing};

pub(crate) struct SpacedListSkeleton<S: Spacing, Sub: SpacedList<S>> {
	link_lengths: Vec<S>,
	sublists: Vec<Option<Sub>>,
	size: usize,
	depth: usize,
	length: S
}