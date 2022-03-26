use crate::SpacedList;
use crate::SpacedListSkeleton;
use crate::Spacing;

pub struct FilledSpacedList<S: Spacing> {
    skeleton: SpacedListSkeleton<S>
}

impl<S: Spacing> SpacedList<S> for FilledSpacedList<S> {

}