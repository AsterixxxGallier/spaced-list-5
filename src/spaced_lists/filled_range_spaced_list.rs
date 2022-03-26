use crate::SpacedList;
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct FilledRangeSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S>
}

impl<S: Spacing> SpacedList<S> for FilledRangeSpacedList<S> {

}