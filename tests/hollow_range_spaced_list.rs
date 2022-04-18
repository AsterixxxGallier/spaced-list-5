use std::fmt::{Display};
use itertools::Itertools;

use spaced_list_5::{HollowRangeSpacedList, RangeIter, Spacing};

fn print<'a, S>(iter: RangeIter<S, HollowRangeSpacedList<S>>)
    where S: 'a + Spacing + Display {
    for (start, end) in iter {
        print!("{}->{}  ", start.position(), end.position());
    }
    println!();
}

#[test]
fn ranges() {
    let ranges = [(1, 2), (5, 3), (10, 1)];
    for ranges in ranges.into_iter().permutations(ranges.len()).skip(2) {
        println!("{:?}", ranges);
        let mut list: HollowRangeSpacedList<u64> = HollowRangeSpacedList::new();
        for (start, end) in ranges {
            list.insert_range(start, end);
        }
        print(list.iter());

        // region 0
        let query_pos = 0;
        assert!(list.range_starting_before(query_pos).is_none());
        assert!(list.range_starting_at_or_before(query_pos).is_none());
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 1);

        assert!(list.range_ending_before(query_pos).is_none());
        assert!(list.range_ending_at_or_before(query_pos).is_none());
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 3);
        // endregion

        // region 1
        let query_pos = 1;
        assert!(list.range_starting_before(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_at(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 5);

        assert!(list.range_ending_before(query_pos).is_none());
        assert!(list.range_ending_at_or_before(query_pos).is_none());
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 3);
        // endregion

        // region 2
        let query_pos = 2;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 1);
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 5);

        assert!(list.range_ending_before(query_pos).is_none());
        assert!(list.range_ending_at_or_before(query_pos).is_none());
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 3);
        // endregion

        // region 3
        let query_pos = 3;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 1);
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 5);

        assert!(list.range_ending_before(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_at(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 8);
        // endregion

        // region 4
        let query_pos = 4;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 1);
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 5);

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 3);
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 8);
        // endregion

        // region 5
        let query_pos = 5;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 1);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_at(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 10);

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 3);
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 8);
        // endregion

        // region 6
        let query_pos = 6;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 5);
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 10);

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 3);
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 8);
        // endregion

        // region 7
        let query_pos = 7;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 5);
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 10);

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 3);
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 8);
        // endregion

        // region 8
        let query_pos = 8;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 5);
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 10);

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 3);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_at(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 11);
        // endregion

        // region 9
        let query_pos = 9;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 5);
        assert!(list.range_starting_at(query_pos).is_none());
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_after(query_pos).unwrap().position(), 10);

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 8);
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 11);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 11);
        // endregion

        // region 10
        let query_pos = 10;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 5);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_at(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_at_or_after(query_pos).unwrap().position(), 10);
        assert!(list.range_starting_after(query_pos).is_none());

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 8);
        assert!(list.range_ending_at(query_pos).is_none());
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 11);
        assert_eq!(list.range_ending_after(query_pos).unwrap().position(), 11);
        // endregion

        // region 11
        let query_pos = 11;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 10);
        assert!(list.range_starting_at(query_pos).is_none());
        assert!(list.range_starting_at_or_after(query_pos).is_none());
        assert!(list.range_starting_after(query_pos).is_none());

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 8);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 11);
        assert_eq!(list.range_ending_at(query_pos).unwrap().position(), 11);
        assert_eq!(list.range_ending_at_or_after(query_pos).unwrap().position(), 11);
        assert!(list.range_ending_after(query_pos).is_none());
        // endregion

        // region 12
        let query_pos = 12;
        assert_eq!(list.range_starting_before(query_pos).unwrap().position(), 10);
        assert_eq!(list.range_starting_at_or_before(query_pos).unwrap().position(), 10);
        assert!(list.range_starting_at(query_pos).is_none());
        assert!(list.range_starting_at_or_after(query_pos).is_none());
        assert!(list.range_starting_after(query_pos).is_none());

        assert_eq!(list.range_ending_before(query_pos).unwrap().position(), 11);
        assert_eq!(list.range_ending_at_or_before(query_pos).unwrap().position(), 11);
        assert!(list.range_ending_at(query_pos).is_none());
        assert!(list.range_ending_at_or_after(query_pos).is_none());
        assert!(list.range_ending_after(query_pos).is_none());
        // endregion
    }
}