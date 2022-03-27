use std::marker::PhantomData;

use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

use crate::{Index, SpacedList, Spacing};

pub struct SpacedListSkeleton<S: Spacing, Sub: SpacedList<S>> {
	link_lengths: Vec<S>,
	sublists: Vec<Option<Sub>>,
	size: usize,
	depth: usize,
	length: S,
}

impl<S: Spacing, Sub: SpacedList<S>> SpacedListSkeleton<S, Sub> {
	fn link_length(&self, index: Index) -> S {
		let mut degree = 0;
		let mut skeleton = self;
		for level in 0..index.sublist_depth - 1 {
			let slice: &BitSlice<usize, Msb0> = &index.bits.view_bits()[degree..degree + skeleton.depth];
			let local_index: usize = slice.load();
			skeleton = skeleton.sublist(local_index).as_ref().unwrap().skeleton();
			degree += skeleton.depth;
		}
		let slice: &BitSlice<usize, Msb0> = &index.bits.view_bits()[degree..degree + skeleton.depth];
		let local_index: usize = slice.load();
		skeleton.link_lengths[local_index]
	}

	fn sublist(&self, index: usize) -> &Option<Sub> {
		&self.sublists[index]
	}
}

#[cfg(test)]
mod tests {
	use crate::HollowSpacedList;

	#[test]
	fn test_link_length_accessor() {
		// let list =
	}
}