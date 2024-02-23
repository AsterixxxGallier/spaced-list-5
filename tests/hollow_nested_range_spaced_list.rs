use std::fmt::Display;
use spaced_list_5::{BoundType, HollowNestedRangeSpacedList, HollowPosition, NestedRange, Spacing};

fn print<'a, S>(iter: impl Iterator<Item=HollowPosition<NestedRange, S>>)
    where S: 'a + Spacing + Display {
    for bound in iter {
        let position = bound.position();
        match bound.bound_type() {
            BoundType::Start => print!("{}->{}  ", position, bound.into_range().1.position()),
            BoundType::End => print!("{}<-{}  ", bound.into_range().0.position(), position),
        }
    }
    println!();
}

#[test]
fn test() {
    let mut list = HollowNestedRangeSpacedList::new();
    list.insert(0, 20);
    list.insert(5, 8);
    list.insert(12, 15);
    list.insert(13, 14);
    print(list.iter());
    assert_eq!(list.starting_at_or_after(5).unwrap().position(), 5);
    assert_eq!(list.starting_at_or_after(6).unwrap().position(), 12);
    assert_eq!(list.ending_at_or_after(6).unwrap().position(), 8);
    assert_eq!(list.ending_at_or_after(10).unwrap().position(), 14);
    assert_eq!(list.starting_before(10).unwrap().position(), 5);
    assert_eq!(list.starting_before(14).unwrap().position(), 13);
    assert_eq!(list.ending_after(14).unwrap().position(), 15);
    assert_eq!(list.ending_after(15).unwrap().position(), 20);
    assert_eq!(list.starting_at(0).unwrap().position(), 0);
}