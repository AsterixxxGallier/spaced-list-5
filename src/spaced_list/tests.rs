#![cfg(test)]

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::{HollowSpacedList, SpacedList};

#[test]
fn grow() {
    let mut list: HollowSpacedList<u32> = HollowSpacedList::new();

    assert_eq!(list.sublists().len(), 0);
    assert_eq!(list.link_capacity(), 0);
    assert_eq!(list.depth(), 0);

    list.grow();

    assert_eq!(list.sublists().len(), 1);
    assert_eq!(list.link_capacity(), 1);
    assert_eq!(list.depth(), 1);

    list.grow();

    assert_eq!(list.sublists().len(), 2);
    assert_eq!(list.link_capacity(), 2);
    assert_eq!(list.depth(), 2);

    list.grow();

    assert_eq!(list.sublists().len(), 4);
    assert_eq!(list.link_capacity(), 4);
    assert_eq!(list.depth(), 3);

    list.grow();

    assert_eq!(list.sublists().len(), 8);
    assert_eq!(list.link_capacity(), 8);
    assert_eq!(list.depth(), 4);
}

#[test]
fn inflate_deflate() {
    let mut list: HollowSpacedList<u32> = HollowSpacedList::new();

    list.grow();
    list.grow();
    list.grow();
    list.set_size_to_capacity();

    assert_eq!(list.depth(), 3);
    assert_eq!(list.length(), 0);
    assert_eq!(list.link_capacity(), 4);

    list.inflate_at(0, 1);
    assert_eq!(list.link_lengths(), &vec![1, 1, 0, 1]);

    list.inflate_at(0, 3);
    assert_eq!(list.link_lengths(), &vec![4, 4, 0, 4]);

    unsafe { list.deflate_at_unchecked(0, 2); }
    assert_eq!(list.link_lengths(), &vec![2, 2, 0, 2]);

    list.inflate_at(1, 3);
    assert_eq!(list.link_lengths(), &vec![2, 5, 0, 5]);

    list.inflate_at(2, 1);
    assert_eq!(list.link_lengths(), &vec![2, 5, 1, 6]);

    list.inflate_at(0, 0);
    list.inflate_at(1, 0);
    list.inflate_at(2, 0);
    list.inflate_at(3, 0);
    assert_eq!(list.link_lengths(), &vec![2, 5, 1, 6]);

    list.deflate_at(0, 0);
    list.deflate_at(1, 0);
    list.deflate_at(2, 0);
    list.deflate_at(3, 0);
    assert_eq!(list.link_lengths(), &vec![2, 5, 1, 6]);
}

#[test]
#[should_panic(expected = "index")]
fn bad_inflate_should_panic_0() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    list.set_size_to_capacity();
    list.inflate_at(0, 1);
}

#[test]
#[should_panic(expected = "negative")]
fn bad_inflate_should_panic_1() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    list.grow();
    list.set_size_to_capacity();
    list.inflate_at(0, -1);
}

#[test]
#[should_panic(expected = "index")]
fn bad_inflate_should_panic_2() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    list.grow();
    list.set_size_to_capacity();
    list.inflate_at(1, 0);
}

#[test]
#[should_panic(expected = "index")]
fn bad_deflate_should_panic_0() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    list.set_size_to_capacity();
    list.deflate_at(0, 0);
}

#[test]
#[should_panic(expected = "negative")]
fn bad_deflate_should_panic_1() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    list.grow();
    list.set_size_to_capacity();
    list.deflate_at(0, -1);
}

#[test]
#[should_panic(expected = "negative")]
fn bad_deflate_should_panic_2() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    list.grow();
    list.grow();
    list.set_size_to_capacity();
    list.inflate_at(0, 1);
    list.deflate_at(1, -1);
}

#[test]
#[should_panic(expected = "below zero")]
fn bad_deflate_should_panic_3() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    list.grow();
    list.grow();
    list.grow();
    list.set_size_to_capacity();
    list.inflate_at(0, 1);
    list.inflate_at(1, 2);
    list.inflate_at(2, 1);
    list.deflate_at(0, 1);
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
    let max = 1 << 16;
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