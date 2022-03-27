use crate::{SpacedList, SpacedListSkeleton, Spacing};

pub trait CrateSpacedList<S: Spacing>: Sized {
	fn skeleton(&self) -> &SpacedListSkeleton<S, Self> where Self: SpacedList<S>;

	fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self> where Self: SpacedList<S>;
}