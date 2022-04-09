use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

pub use filled_range_spaced_list::FilledRangeSpacedList;
pub use filled_spaced_list::FilledSpacedList;
pub use hollow_range_spaced_list::HollowRangeSpacedList;
pub use hollow_spaced_list::HollowSpacedList;
pub use iteration::node::Iter;
pub use positions::node::Position;
pub(crate) use spaced_list::SpacedList;
pub(crate) use skeleton::SpacedListSkeleton;

pub trait Spacing = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy;

macro_rules! default_as_new {
    () => {
        pub fn new() -> Self {
            default()
        }
    };
}

mod skeleton;

mod spaced_list;

mod hollow_spaced_list;

mod filled_spaced_list;

mod hollow_range_spaced_list;

mod filled_range_spaced_list;

mod traversal;

mod iteration;

mod positions;