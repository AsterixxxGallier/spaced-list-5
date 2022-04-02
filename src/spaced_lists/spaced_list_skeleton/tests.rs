#![cfg(test)]

use rand::{Rng, thread_rng};
use crate::{HollowSpacedList, SpacedList, SpacedListSkeleton};

#[test]
fn grow() {
    let mut list: HollowSpacedList<u32> = HollowSpacedList::new();
    let mut skeleton = list.skeleton_mut();

    assert_eq!(skeleton.sublists.len(), 0);
    assert_eq!(skeleton.size(), 0);
    assert_eq!(skeleton.depth(), 0);

    skeleton.grow();

    assert_eq!(skeleton.sublists.len(), 1);
    assert_eq!(skeleton.size(), 1);
    assert_eq!(skeleton.depth(), 1);

    skeleton.grow();

    assert_eq!(skeleton.sublists.len(), 2);
    assert_eq!(skeleton.size(), 2);
    assert_eq!(skeleton.depth(), 2);

    skeleton.grow();

    assert_eq!(skeleton.sublists.len(), 4);
    assert_eq!(skeleton.size(), 4);
    assert_eq!(skeleton.depth(), 3);

    skeleton.grow();

    assert_eq!(skeleton.sublists.len(), 8);
    assert_eq!(skeleton.size(), 8);
    assert_eq!(skeleton.depth(), 4);
}

#[test]
fn inflate_deflate() {
    let mut list: HollowSpacedList<u32> = HollowSpacedList::new();
    let mut skeleton = list.skeleton_mut();

    skeleton.grow();
    skeleton.grow();
    skeleton.grow();

    assert_eq!(skeleton.depth, 3);
    assert_eq!(skeleton.length, 0);
    assert_eq!(skeleton.size(), 4);
    assert_eq!(skeleton.size, 4);

    skeleton.inflate_at(0, 1);
    assert_eq!(skeleton.link_lengths, vec![1, 1, 0, 1]);

    skeleton.inflate_at(0, 3);
    assert_eq!(skeleton.link_lengths, vec![4, 4, 0, 4]);

    unsafe { skeleton.deflate_at(0, 2); }
    assert_eq!(skeleton.link_lengths, vec![2, 2, 0, 2]);

    skeleton.inflate_at(1, 3);
    assert_eq!(skeleton.link_lengths, vec![2, 5, 0, 5]);

    skeleton.inflate_at(2, 1);
    assert_eq!(skeleton.link_lengths, vec![2, 5, 1, 6]);
}

#[test]
fn random_insertions() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    let mut rng = thread_rng();
    for n in 0..100 {
        let pos = rng.gen_range(0..100);
        // println!("inserting node at {}", pos);
        // println!("{:?}", pos);
        list.insert_node(pos);
        // list.insert_node(n);
        // println!("{:?}", list.skeleton().format(
        //     true,
        //     true,
        //     true,
        //     4,
        //     vec![],
        //     vec![]
        // ));
        // assert_eq!(list.node_before(pos + 1).unwrap().position, pos);
        // assert_eq!(list.node_at_or_before(pos).unwrap().position, pos);
        assert_eq!(list.node_at(pos).unwrap().position, pos);
        // assert_eq!(list.node_at_or_after(pos).unwrap().position, pos);
        // assert_eq!(list.node_after(pos - 1).unwrap().position, pos);
    }

    // let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    // list.insert_node(1);
    // list.insert_node(2);
    // list.insert_node(3);
    // list.insert_node(4);
    // list.insert_node(6);
    // // I expect the sublist to be inserted after position 4 = index 4, but it actually goes to index 5?
    // list.insert_node(5);
    // println!("{:?}", list.node_at_or_before(5));
}