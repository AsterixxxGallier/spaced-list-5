use crate::spaced_lists::CrateSpacedList;
use crate::SpacedList;
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S, Self>
}

impl<S: Spacing> CrateSpacedList<S> for HollowRangeSpacedList<S> {
    fn skeleton(&self) -> &SpacedListSkeleton<S, Self> where Self: SpacedList<S> {
        &self.skeleton
    }

    fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self> where Self: SpacedList<S> {
        &mut self.skeleton
    }
}

impl<S: Spacing> SpacedList<S> for HollowRangeSpacedList<S> {

}