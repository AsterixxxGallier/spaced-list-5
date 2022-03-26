use std::marker::PhantomData;
use crate::Spacing;

pub(crate) struct SpacedListSkeleton<S: Spacing> {
	phantom: PhantomData<S>
}