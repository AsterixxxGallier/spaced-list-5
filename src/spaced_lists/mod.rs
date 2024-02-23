use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub(crate) enum SpacingOperation { Increase, Decrease }

impl SpacingOperation {
    fn complement(self) -> Self {
        match self {
            SpacingOperation::Increase => SpacingOperation::Decrease,
            SpacingOperation::Decrease => SpacingOperation::Increase,
        }
    }
}

impl Display for SpacingOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            SpacingOperation::Increase => f.write_str("increase"),
            SpacingOperation::Decrease => f.write_str("decrease"),
        }
    }
}

#[derive(Error, Debug)]
pub enum SpacingError<S: crate::Spacing> {
    #[error("Cannot {operation} spacing after position {position}, \
             as that position is at or after the end of this list.")]
    PositionAtOrAfterList {
        operation: SpacingOperation,
        position: S,
    },
    #[error("Cannot {operation} spacing before position {position}, \
             as that position is after the end of this list.")]
    PositionAfterList {
        operation: SpacingOperation,
        position: S,
    },
    #[error("Cannot {operation} spacing by {amount}, as it is a negative amount. \
             Explicitly {} spacing for that.", operation.complement())]
    AmountNegative {
        operation: SpacingOperation,
        amount: S,
    },
    #[error("The spacing at position {position} is {spacing}, which is less than {amount}. \
             It cannot be decreased by this amount without becoming negative.")]
    SpacingNotLargeEnough {
        position: S,
        amount: S,
        spacing: S,
    },
}

// TODO panicking non-try versions of try_ functions

macro_rules! spacing_functions {
    () => {
        pub fn increase_spacing_after(&mut self, position: S, spacing: S) {
            Skeleton::inflate_after(self.skeleton.clone(), position, spacing);
        }

        pub fn increase_spacing_before(&mut self, position: S, spacing: S) {
            Skeleton::inflate_before(self.skeleton.clone(), position, spacing);
        }

        pub fn decrease_spacing_after(&mut self, position: S, spacing: S) {
            Skeleton::deflate_after(self.skeleton.clone(), position, spacing);
        }

        pub fn decrease_spacing_before(&mut self, position: S, spacing: S) {
            Skeleton::deflate_before(self.skeleton.clone(), position, spacing);
        }


        pub fn try_increase_spacing_after(&mut self, position: S, spacing: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_inflate_after(self.skeleton.clone(), position, spacing)?;
            Ok(())
        }

        pub fn try_increase_spacing_before(&mut self, position: S, spacing: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_inflate_before(self.skeleton.clone(), position, spacing)?;
            Ok(())
        }

        pub fn try_decrease_spacing_after(&mut self, position: S, spacing: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_deflate_after(self.skeleton.clone(), position, spacing)?;
            Ok(())
        }

        pub fn try_decrease_spacing_before(&mut self, position: S, spacing: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_deflate_before(self.skeleton.clone(), position, spacing)?;
            Ok(())
        }
    }
}

macro_rules! trivial_accessors {
    () => {
        #[must_use]
        pub fn size(&self) -> usize {
            self.size
        }

        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        #[must_use]
        pub fn length(&self) -> S {
            self.skeleton.borrow().length()
        }

        #[must_use]
        pub fn start(&self) -> S {
            self.skeleton.borrow().offset()
        }

        #[must_use]
        pub fn end(&self) -> S {
            self.skeleton.borrow().last_position()
        }
    }
}

pub mod spaced_list;
pub mod range_spaced_list;
pub mod nested_range_spaced_list;
pub mod hollow_spaced_list;
pub mod hollow_range_spaced_list;
pub mod hollow_nested_range_spaced_list;
