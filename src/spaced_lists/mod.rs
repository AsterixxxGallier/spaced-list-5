use crate::skeleton::flate::FlateError;

// TODO panicking non-try versions of try_ functions

// basically FlateError but more user-friendly (fit for public visibility)
#[derive(Debug)]
pub enum SpacingError {
    /// "Cannot increase/decrease spacing after the given position, as that position is at or after the end of this list"
    PositionAtOrAfterList,
    /// "Cannot increase/decrease spacing before the given position, as that position is after the end of this list"
    PositionAfterList,
    /// "Cannot increase/decrease spacing by a negative amount, explicitly decrease/increase spacing for that"
    AmountNegative,
    /// "The spacing at the given position is not large enough to be decreased by the given amount"
    SpacingNotLargeEnough,
}

impl From<FlateError> for SpacingError {
    fn from(value: FlateError) -> Self {
        match value {
            FlateError::PositionAtOrAfterList => SpacingError::PositionAtOrAfterList,
            FlateError::PositionAfterList => SpacingError::PositionAfterList,
            FlateError::AmountNegative => SpacingError::AmountNegative,
            FlateError::DeflationBelowZero => SpacingError::SpacingNotLargeEnough,
        }
    }
}

macro_rules! spacing_methods {
    () => {
        // "?; Ok(())" structure for automatic error type conversion via the ? operator

        pub fn try_increase_spacing_after(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
            Skeleton::try_inflate_after(self.skeleton.clone(), position, spacing)?;
            Ok(())
        }

        pub fn try_increase_spacing_before(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
            Skeleton::try_inflate_before(self.skeleton.clone(), position, spacing)?;
            Ok(())
        }

        pub fn try_decrease_spacing_after(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
            Skeleton::try_deflate_after(self.skeleton.clone(), position, spacing)?;
            Ok(())
        }

        pub fn try_decrease_spacing_before(&mut self, position: S, spacing: S) -> Result<(), SpacingError> {
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

pub(crate) use spacing_methods;
pub(crate) use trivial_accessors;

pub mod spaced_list;
pub mod range_spaced_list;
pub mod nested_range_spaced_list;
pub mod hollow_spaced_list;
pub mod hollow_range_spaced_list;
pub mod hollow_nested_range_spaced_list;
