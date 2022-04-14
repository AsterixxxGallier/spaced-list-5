#![cfg(test)]

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::{HollowSpacedList, SpacedList};

#[test]
fn grow() {
    let mut list: HollowSpacedList<u32> = HollowSpacedList::new();
    let mut skeleton = list;

    assert_eq!(skeleton.sublists().len(), 0);
    assert_eq!(skeleton.link_capacity(), 0);
    assert_eq!(skeleton.depth(), 0);

    skeleton.grow();

    assert_eq!(skeleton.sublists().len(), 1);
    assert_eq!(skeleton.link_capacity(), 1);
    assert_eq!(skeleton.depth(), 1);

    skeleton.grow();

    assert_eq!(skeleton.sublists().len(), 2);
    assert_eq!(skeleton.link_capacity(), 2);
    assert_eq!(skeleton.depth(), 2);

    skeleton.grow();

    assert_eq!(skeleton.sublists().len(), 4);
    assert_eq!(skeleton.link_capacity(), 4);
    assert_eq!(skeleton.depth(), 3);

    skeleton.grow();

    assert_eq!(skeleton.sublists().len(), 8);
    assert_eq!(skeleton.link_capacity(), 8);
    assert_eq!(skeleton.depth(), 4);
}

#[test]
fn inflate_deflate() {
    let mut list: HollowSpacedList<u32> = HollowSpacedList::new();
    let mut skeleton = list;

    skeleton.grow();
    skeleton.grow();
    skeleton.grow();
    skeleton.set_size_to_capacity();

    assert_eq!(skeleton.depth(), 3);
    assert_eq!(skeleton.length(), 0);
    assert_eq!(skeleton.link_capacity(), 4);

    skeleton.inflate_at(0, 1);
    assert_eq!(skeleton.link_lengths(), &vec![1, 1, 0, 1]);

    skeleton.inflate_at(0, 3);
    assert_eq!(skeleton.link_lengths(), &vec![4, 4, 0, 4]);

    unsafe { skeleton.deflate_at_unchecked(0, 2); }
    assert_eq!(skeleton.link_lengths(), &vec![2, 2, 0, 2]);

    skeleton.inflate_at(1, 3);
    assert_eq!(skeleton.link_lengths(), &vec![2, 5, 0, 5]);

    skeleton.inflate_at(2, 1);
    assert_eq!(skeleton.link_lengths(), &vec![2, 5, 1, 6]);

    skeleton.inflate_at(0, 0);
    skeleton.inflate_at(1, 0);
    skeleton.inflate_at(2, 0);
    skeleton.inflate_at(3, 0);
    assert_eq!(skeleton.link_lengths(), &vec![2, 5, 1, 6]);

    skeleton.deflate_at(0, 0);
    skeleton.deflate_at(1, 0);
    skeleton.deflate_at(2, 0);
    skeleton.deflate_at(3, 0);
    assert_eq!(skeleton.link_lengths(), &vec![2, 5, 1, 6]);
}

#[test]
#[should_panic(expected = "index")]
fn bad_inflate_should_panic_0() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut skeleton = list;
    skeleton.set_size_to_capacity();
    skeleton.inflate_at(0, 1);
}

#[test]
#[should_panic(expected = "negative")]
fn bad_inflate_should_panic_1() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut skeleton = list;
    skeleton.grow();
    skeleton.set_size_to_capacity();
    skeleton.inflate_at(0, -1);
}

#[test]
#[should_panic(expected = "index")]
fn bad_inflate_should_panic_2() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut skeleton = list;
    skeleton.grow();
    skeleton.set_size_to_capacity();
    skeleton.inflate_at(1, 0);
}

#[test]
#[should_panic(expected = "index")]
fn bad_deflate_should_panic_0() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut skeleton = list;
    skeleton.set_size_to_capacity();
    skeleton.deflate_at(0, 0);
}

#[test]
#[should_panic(expected = "negative")]
fn bad_deflate_should_panic_1() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut skeleton = list;
    skeleton.grow();
    skeleton.set_size_to_capacity();
    skeleton.deflate_at(0, -1);
}

#[test]
#[should_panic(expected = "negative")]
fn bad_deflate_should_panic_2() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut skeleton = list;
    skeleton.grow();
    skeleton.grow();
    skeleton.set_size_to_capacity();
    skeleton.inflate_at(0, 1);
    skeleton.deflate_at(1, -1);
}

#[test]
#[should_panic(expected = "below zero")]
fn bad_deflate_should_panic_3() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut skeleton = list;
    skeleton.grow();
    skeleton.grow();
    skeleton.grow();
    skeleton.set_size_to_capacity();
    skeleton.inflate_at(0, 1);
    skeleton.inflate_at(1, 2);
    skeleton.inflate_at(2, 1);
    skeleton.deflate_at(0, 1);
}

#[test]
fn random_insertions() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    let mut rng = StdRng::seed_from_u64(0);
    // performance for random:
    // 1 << 20: 1.4 s = 1.3 µs/node
    // 1 << 21: 3.4 s = 1.6 µs/node
    // 1 << 22: 7.8 s = 1.9 µs/node
    // 1 << 23:  19 s = 2.3 µs/node
    // 1 << 24:  45 s = 2.7 µs/node
    // 1 << 25: 125 s = 3.7 µs/node
    // 1 << 26: doesn't stop, apparently
    let max = 1 << 20;
    for n in 0..max {
        if n % 100000 == 0 {
            println!("n = {}", n);
        }
        let pos = rng.gen_range(0..100_000_000_000);
        // let pos = max - n;
        // println!("inserting node at {}, iteration: {}", pos, n);
        list.insert_node(pos);
        // println!("{:?}", list.format(
        //     true,
        //     true,
        //     true,
        //     4,
        //     vec![],
        //     vec![]
        // ));
        assert_eq!(list.node_before(pos + 1).unwrap().position(), pos);
        assert_eq!(list.node_at_or_before(pos).unwrap().position(), pos);
        assert_eq!(list.node_at(pos).unwrap().position(), pos);
        assert_eq!(list.node_at_or_after(pos).unwrap().position(), pos);
        assert_eq!(list.node_after(pos - 1).unwrap().position(), pos);
    }
    println!("{}", list.node_size_deep())

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