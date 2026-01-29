// cache system design

// Q1. LRU Cache
// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

// Implement the LRUCache class:

// - LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
// - int get(int key) Return the value of the key if the key exists, otherwise return -1.
// - void put(int key, int value) Update the value of the key if the key exists. Otherwise,
// add the key-value pair to the cache. If the number of keys exceeds the capacity from this
// operation, evict the least recently used key.

// The functions get and put must each run in O(1) average time complexity.

use hashlink::LinkedHashMap;

struct LRUCache {
    capacity: usize,
    cache: LinkedHashMap<i32, i32>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: LinkedHashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(val) = self.cache.remove(&key) {
            self.cache.insert(key, val);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.cache.remove(&key);
            self.cache.insert(key, value);
        } else if self.cache.len() >= self.capacity {
            self.cache.pop_front();
            self.cache.insert(key, value);
        } else {
            self.cache.insert(key, value);
            if self.cache.len() >= self.capacity {
                self.cache.pop_front();
            }
        }
    }
}

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
