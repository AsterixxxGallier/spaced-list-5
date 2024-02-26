macro_rules! iter_functions {
    (Node; $position:ty) => {
        pub fn iter(&self) -> impl Iterator<Item=$position> {
            ForwardsIter::from_start(self.skeleton.clone()).filter(|pos| pos.ephemeral().element().is_some()).map_into()
        }

        // covered by a to-do item somewhere else
        #[allow(clippy::should_implement_trait)]
        pub fn into_iter(self) -> impl Iterator<Item=$position> {
            ForwardsIter::from_start(self.skeleton).filter(|pos| pos.ephemeral().element().is_some()).map_into()
        }

        pub fn iter_backwards(&self) -> impl Iterator<Item=$position> {
            BackwardsIter::from_end(self.skeleton.clone()).filter(|pos| pos.ephemeral().element().is_some()).map_into()
        }

        pub fn into_iter_backwards(self) -> impl Iterator<Item=$position> {
            BackwardsIter::from_end(self.skeleton).filter(|pos| pos.ephemeral().element().is_some()).map_into()
        }
    };
    (Range; $position:ty) => {
        iter_functions!(Node; $position);

        pub fn iter_ranges(&self) -> impl Iterator<Item=($position, $position)> {
            self.iter().tuples()
        }

        pub fn into_iter_ranges(self) -> impl Iterator<Item=($position, $position)> {
            self.into_iter().tuples()
        }

        pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=($position, $position)> {
            self.iter().tuples()
        }

        pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=($position, $position)> {
            self.into_iter().tuples()
        }
    };
    (NestedRange; $position:ty) => {
        iter_functions!(Node; $position);

        // TODO nested iter_ranges
    }
}

pub(super) use iter_functions;