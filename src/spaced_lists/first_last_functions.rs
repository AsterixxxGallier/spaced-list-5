macro_rules! first_last_functions {
    ($position_ident:ident, $position:ty) => {
        #[must_use]
        pub fn first(&self) -> Option<$position> {
            if self.is_empty() {
                None
            } else {
                Some($position_ident::at_start(self.skeleton.clone()))
            }
        }

        #[must_use]
        pub fn last(&self) -> Option<$position> {
            if self.is_empty() {
                None
            } else {
                Some($position_ident::at_end(self.skeleton.clone()))
            }
        }
    };
}

pub(super) use first_last_functions;