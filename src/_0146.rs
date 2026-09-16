#[allow(unused)]
struct Solution;

use std::collections::HashMap;
#[derive(Clone, Copy)]
struct LinkedList {
    prev: usize,
    next: usize,
}

struct LRUCache {
    capacity: usize,
    ptr_list: Vec<LinkedList>,    // (prev, next)
    kv_list: Vec<(i32, i32)>,     // (key, value)
    hashmap: HashMap<i32, usize>, // key -> index
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let mut ptr_list = vec![LinkedList { prev: 0, next: 0 }; capacity + 1];
        ptr_list[capacity] = LinkedList {
            prev: capacity,
            next: capacity,
        }; // let the last ptr be (tail, head)
        Self {
            capacity,
            ptr_list,
            kv_list: vec![(0, 0); capacity],
            hashmap: HashMap::with_capacity(capacity),
        }
    }

    #[inline]
    fn detach(&mut self, index: usize) {
        let LinkedList { prev, next } = self.ptr_list[index];
        self.ptr_list[prev].next = next;
        self.ptr_list[next].prev = prev;
    }

    #[inline]
    fn add_to_head(&mut self, index: usize) {
        let LinkedList { next: head, .. } = self.ptr_list[self.capacity];
        self.ptr_list[head].prev = index;
        self.ptr_list[self.capacity].next = index;
        self.ptr_list[index] = LinkedList {
            prev: self.capacity,
            next: head,
        };
    }

    #[inline]
    fn move_to_head(&mut self, index: usize) {
        assert!(index < self.capacity);
        self.detach(index);
        self.add_to_head(index);
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&index) = self.hashmap.get(&key) {
            self.move_to_head(index);
            self.kv_list[index].1
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&index) = self.hashmap.get(&key) {
            self.kv_list[index] = (key, value);
            self.move_to_head(index);
            return;
        }

        let mut index = self.hashmap.len();
        if index >= self.capacity {
            index = self.ptr_list[self.capacity].prev; // tail
            let key_to_remove = self.kv_list[index].0;
            self.hashmap.remove(&key_to_remove);
            self.detach(index);
        }

        self.hashmap.insert(key, index);
        self.kv_list[index] = (key, value);
        self.add_to_head(index);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);
        lru_cache.put(3, 3);
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }
}
