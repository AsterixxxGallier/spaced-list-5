use spaced_list_5::HollowSpacedList;

#[test]
fn iterate() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    list.insert_node(13);
    list.insert_node(7);
    list.insert_node(8);
    list.insert_node(15);
    list.insert_node(20);
    list.insert_node(16);
    let mut iter = list.iter();
    assert_eq!(iter.next().unwrap().position(), 7);
    assert_eq!(iter.next().unwrap().position(), 8);
    assert_eq!(iter.next().unwrap().position(), 13);
    assert_eq!(iter.next().unwrap().position(), 15);
    assert_eq!(iter.next().unwrap().position(), 16);
    assert_eq!(iter.next().unwrap().position(), 20);
}

#[test]
fn inflate_deflate() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    list.insert_node(47);
    list.insert_node(5);
    list.insert_node(14);
    list.insert_node(13);
    list.inflate_after(5, 3);
    assert_eq!(list.node_at(5).unwrap().position(), 5);
    assert!(list.node_at(13).is_none());
    assert_eq!(list.node_at(16).unwrap().position(), 16);
    assert_eq!(list.node_at(17).unwrap().position(), 17);
    assert_eq!(list.node_after(20).unwrap().position(), 50);
    list.inflate_after(0, 100);
    assert_eq!(list.node_at(105).unwrap().position(), 105);
    assert_eq!(list.node_at(117).unwrap().position(), 117);
    assert_eq!(list.node_at(150).unwrap().position(), 150);
    list.deflate_after(105, 10);
    assert_eq!(list.node_at(105).unwrap().position(), 105);
    assert_eq!(list.node_at(107).unwrap().position(), 107);
    assert_eq!(list.node_at(140).unwrap().position(), 140);
    list.inflate_before(105, 20);
    assert_eq!(list.node_at(125).unwrap().position(), 125);
    assert_eq!(list.node_at(160).unwrap().position(), 160);
    list.deflate_before(130, 10);
    assert_eq!(list.node_at(125).unwrap().position(), 125);
    assert_eq!(list.node_at(150).unwrap().position(), 150);
}

#[test]
fn queries() {
    let mut list: HollowSpacedList<i64> = HollowSpacedList::new();
    list.insert_node(1);
    list.insert_node(5);
    list.insert_node(3);
    list.insert_node(0);

    // region -1
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
    // endregion

    // region 0
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
    // endregion

    // region 1
    let query_pos = 1;

    let pos = list.node_before(query_pos).unwrap();
    assert_eq!(pos.position(), 0);

    let pos = list.node_at_or_before(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_at(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    let pos = list.node_at_or_after(query_pos).unwrap();
    assert_eq!(pos.position(), 1);

    // TODO fix this
    let pos = list.node_after(query_pos).unwrap();
    assert_eq!(pos.position(), 3);
    // endregion

    // region 2
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
    // endregion

    // region 3
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
    // endregion

    // region 4
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
    // endregion

    // region 5
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
    // endregion

    // region 6
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
    // endregion
}
