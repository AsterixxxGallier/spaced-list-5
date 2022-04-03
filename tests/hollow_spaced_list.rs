#![allow(unreachable_code)]

use spaced_list_5::HollowSpacedList;

#[test]
fn test() {
    let mut list: HollowSpacedList<i64> = HollowSpacedList::new();
    list.insert_node(1);
    list.insert_node(5);
    list.insert_node(3);

    let pos = list.node_before(-1);
    assert!(pos.is_none());

    let pos = list.node_at_or_before(-1);
    assert!(pos.is_none());

    let pos = list.node_at(-1);
    assert!(pos.is_none());

    let pos = list.node_at_or_after(-1).unwrap();
    assert_eq!(pos.position(), 0);

    let pos = list.node_after(-1).unwrap();
    assert_eq!(pos.position(), 0);
}