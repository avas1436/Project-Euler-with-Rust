// cache system design

// Q1. LRU Cache
// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

// Implement the LRUCache class:

// - LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
// - int get(int key) Return the value of the key if the key exists, otherwise return -1.
// - void put(int key, int value) Update the value of the key if the key exists. Otherwise,
// add the key-value pair to the cache. If the number of keys exceeds the capacity from this
// operation, evict the least recently used key.

use std::{collections::HashMap, usize};

// The functions get and put must each run in O(1) average time complexity.
struct Node {
    value: usize,
    prev: Option<usize>,
    next: Option<usize>,
}
struct DoublyLinkList {
    nodes: Vec<Node>,
    head: Option<usize>,
    tail: Option<usize>,
}
impl DoublyLinkList {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            tail: None,
        }
    }
    //     fn add_node(&mut self, node: Node) -> Self {
    //         match self.head {
    //             Some(val) => {

    //             }
    //             None => {
    //                 self.head = Some(1);
    //                 self.tail = Some(1);
    //             }
    //         }
    //     }
}
struct LRUCache {
    cache: HashMap<usize, Node>,
    capacity: usize,
}

// impl LRUCache {
//     fn new(capacity: i32) -> Self {}

//     fn get(&mut self, key: i32) -> i32 {}

//     fn put(&mut self, key: i32, value: i32) {}
// }

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
    let commands = vec![
        "LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get",
    ];
    let args: Vec<Vec<i32>> = vec![
        vec![2],
        vec![1, 1],
        vec![2, 2],
        vec![1],
        vec![3, 3],
        vec![2],
        vec![4, 4],
        vec![1],
        vec![3],
        vec![4],
    ];

    let mut cache = LRUCache::new(args[0][0] as usize);
    let mut results: Vec<Option<i32>> = Vec::new();

    for (i, cmd) in commands.iter().enumerate().skip(1) {
        match *cmd {
            "put" => {
                cache.put(args[i][0], args[i][1]);
                results.push(None);
            }
            "get" => {
                let val = cache.get(args[i][0]);
                results.push(Some(val));
            }
            _ => {}
        }
    }

    println!("{:?}", results);
}
