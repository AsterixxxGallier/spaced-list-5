use std::ops::Neg;

use crate::{Index, Spacing, Todo};
use crate::spaced_lists::CrateSpacedList;

pub trait SpacedList<S: Spacing>: CrateSpacedList<S> {
	fn append_node(&mut self, distance: S) {
		todo!()
	}

	fn insert_node(&mut self, position: S) {
		todo!()
	}

	fn inflate_after(&mut self, node_index: Index, amount: S) {
		todo!()
	}

	fn inflate_before(&mut self, node_index: Index, amount: S) {
		todo!()
	}

	fn deflate_after(&mut self, node_index: Index, amount: S) where S: Neg<Output = S> {
		self.inflate_after(node_index, -amount)
	}

	fn deflate_before(&mut self, node_index: Index, amount: S) where S: Neg<Output = S> {
		self.inflate_before(node_index, -amount)
	}

	fn node_before(&self, position: S) -> Todo {
		todo!()
	}

	fn node_at_or_before(&self, position: S) -> Todo {
		todo!()
	}

	fn node_at(&self, position: S) -> Todo {
		todo!()
	}

	fn node_at_or_after(&self, position: S) -> Todo {
		todo!()
	}

	fn node_after(&self, position: S) -> Todo {
		todo!()
	}
}