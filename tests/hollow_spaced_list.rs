#![allow(unreachable_code)]

use spaced_list_5::HollowSpacedList;

#[test]
fn test() {
    let mut list: HollowSpacedList<i64> = HollowSpacedList::new();
    list.insert_node(1);
    list.insert_node(5);
    list.insert_node(3);

    let query_pos = -1;

    let pos = list.node_before(query_pos);
    assert!(pos.is_none());

    let pos = list.node_at_or_before(query_pos);
    assert!(pos.is_none());

    let pos = list.node_at(query_pos);
    assert!(pos.is_none());

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 0);

    let pos = list.node_after(query_pos).unwrap();
    assert_eq!(pos.position(), 0);

    let query_pos = 0;

    let pos = list.node_before(query_pos);
    assert!(pos.is_none());

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 0);

    let pos = list.node_at(query_pos).unwrap();
    assert_eq!(pos.position(), 0);

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 0);

    let pos = list.node_after(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let query_pos = 1;

    let pos = list.node_before(query_pos).unwrap();
    assert_eq!(pos.position(), 0);

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_at(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_after(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let query_pos = 2;

    let pos = list.node_before(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_at(query_pos);
    assert!(pos.is_none());

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let pos = list.node_after(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let query_pos = 3;

    let pos = list.node_before(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let pos = list.node_at(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let pos = list.node_after(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let query_pos = 4;

    let pos = list.node_before(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let pos = list.node_at(query_pos);
    assert!(pos.is_none());

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let pos = list.node_after(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let query_pos = 5;

    let pos = list.node_before(query_pos).unwrap();
    assert_eq!(pos.position(), 3);

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let pos = list.node_at(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let pos = list.node_after(query_pos);
    assert!(pos.is_none());

    let query_pos = 6;

    let pos = list.node_before(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 5);

    let pos = list.node_at(query_pos);
    assert!(pos.is_none());

    let pos = list.node_at_or_after(query_pos);
    assert!(pos.is_none());

    let pos = list.node_after(query_pos);
    assert!(pos.is_none());
}