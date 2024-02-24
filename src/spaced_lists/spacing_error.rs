use thiserror::Error;
use crate::Spacing;

#[derive(Error, Debug)]
pub enum SpacingError<S: Spacing> {
    #[error("Cannot change spacing after position {position}, as that position is at or after the end of this list.")]
    PositionAtOrAfterList {
        position: S,
    },
    #[error("Cannot change spacing before position {position}, as that position is after the end of this list.")]
    PositionAfterList {
        position: S,
    },
    #[error("The spacing at position {position} is {spacing}. It is not large enough to be able to be decreased by {change} without becoming negative.")]
    SpacingNotLargeEnough {
        position: S,
        change: S,
        spacing: S,
    },
}
