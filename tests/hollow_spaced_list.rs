#![allow(unreachable_code)]

use spaced_list_5::HollowSpacedList;

#[test]
fn test() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    list.insert_node(1);
    list.insert_node(5);
    list.insert_node(3);

    let pos = list.node_at(0).unwrap();
    assert_eq!(pos.position, 0);

    let pos = list.node_at(1).unwrap();
    println!("{:?}", pos);
    assert_eq!(pos.position, 1);

    let pos = list.node_at(3).unwrap();
    assert_eq!(pos.position, 3);

    let pos = list.node_at(5).unwrap();
    assert_eq!(pos.position, 5);
}