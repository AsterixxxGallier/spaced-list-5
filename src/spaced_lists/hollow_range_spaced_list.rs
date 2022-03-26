use crate::SpacedList;
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>
}

impl<S: Spacing> SpacedList<S> for HollowRangeSpacedList<S> {

}