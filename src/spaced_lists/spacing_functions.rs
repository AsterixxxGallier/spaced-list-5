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

pub(super) use spacing_functions;