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

pub(super) use trivial_accessors;