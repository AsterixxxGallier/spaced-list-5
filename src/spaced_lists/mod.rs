use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpacingError<S: crate::Spacing> {
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

// TODO more panicking non-try versions of try_ functions

macro_rules! spacing_functions {
    () => {
        pub fn increase_spacing_after(&mut self, position: S, change: S) {
            Skeleton::increase_spacing_after(self.skeleton.clone(), position, change);
        }

        pub fn increase_spacing_before(&mut self, position: S, change: S) {
            Skeleton::increase_spacing_before(self.skeleton.clone(), position, change);
        }

        pub fn decrease_spacing_after(&mut self, position: S, change: S) {
            Skeleton::decrease_spacing_after(self.skeleton.clone(), position, change);
        }

        pub fn decrease_spacing_before(&mut self, position: S, change: S) {
            Skeleton::decrease_spacing_before(self.skeleton.clone(), position, change);
        }


        pub fn try_increase_spacing_after(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_increase_spacing_after(self.skeleton.clone(), position, change)?;
            Ok(())
        }

        pub fn try_increase_spacing_before(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_increase_spacing_before(self.skeleton.clone(), position, change)?;
            Ok(())
        }

        pub fn try_decrease_spacing_after(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_decrease_spacing_after(self.skeleton.clone(), position, change)?;
            Ok(())
        }

        pub fn try_decrease_spacing_before(&mut self, position: S, change: S) -> Result<(), SpacingError<S>> {
            Skeleton::try_decrease_spacing_before(self.skeleton.clone(), position, change)?;
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
