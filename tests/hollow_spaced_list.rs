#![allow(unreachable_code)]

use rand::{Rng, thread_rng};
use spaced_list_5::HollowSpacedList;

#[test]
fn test() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    list.insert_node(1);
    list.insert_node(5);
    list.insert_node(3);

    let pos = list.node_at(0).unwrap();
    assert_eq!(pos.position, 0);
    assert_eq!(pos.index, 0);

    let pos = list.node_at(1).unwrap();
    assert_eq!(pos.position, 1);
    assert_eq!(pos.index, 1);

    let pos = list.node_at(3).unwrap();
    assert_eq!(pos.position, 3);
    assert_eq!(pos.index, 1 + (1 << 2));

    let pos = list.node_at(5).unwrap();
    assert_eq!(pos.position, 5);
    assert_eq!(pos.index, 2);
}