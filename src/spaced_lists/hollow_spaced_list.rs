use crate::SpacedList;
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct HollowSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>
}

impl<S: Spacing> SpacedList<S> for HollowSpacedList<S> {

}