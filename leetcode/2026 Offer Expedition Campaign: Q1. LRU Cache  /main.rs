use std::collections::HashMap;

// LRU implementation using an Arena for the LL

struct Node {
    key: i32,
    value: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

struct LRUCache {
    map: HashMap<i32, usize>,
    capacity: usize,
    arena: Vec<Node>,
    head: Option<usize>,
    tail: Option<usize>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        LRUCache {
            map: HashMap::with_capacity(capacity),
            arena: Vec::with_capacity(capacity),
            capacity,
            head: None,
            tail: None,
        }
    }

    fn swap_to_most_recently_used(&mut self, idx: usize) {
        if self.head == Some(idx) {
            // already the latest used value, early exit
            return;
        }

        let old_head = self.head.unwrap();

        // first, connect the prev and next of the item being taken out of the LL chain, but only if they exist
        let old_item_prev = self.arena[idx].prev;
        let old_item_next = self.arena[idx].next;

        if let Some(old_item_next) = old_item_next {
            self.arena[old_item_next].prev = old_item_prev;
        }
        if let Some(old_item_prev) = old_item_prev { 
            self.arena[old_item_prev].next = old_item_next;
            if old_item_next.is_none() {
                // this is the new tail.
                self.tail = Some(old_item_prev);
            }
        }

        // move the head up one node 
        self.arena[old_head].prev = Some(idx);
        // setup the node as the new head
        self.arena[idx].prev = None;
        self.arena[idx].next = Some(old_head);

        self.head = Some(idx);
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            self.swap_to_most_recently_used(idx);
            self.arena[idx].value
        } else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        // only needs updation
        if let Some(&idx) = self.map.get(&key) {
            self.arena[idx].value = value;
            self.swap_to_most_recently_used(idx);
        } 
        // needs insertion, but under capacity
        else if self.map.len() < self.capacity {
            let insert_idx = self.arena.len();
            self.arena.push(Node {
                key,
                value,
                // since we are checking for next/prev pointers, we can simply leave these as nil 
                // and they should be adjusted by the swap
                next: None,
                prev: None,
            });
            self.map.insert(key, insert_idx);
            if self.map.len() == 1 {
                // first item!
                self.head = Some(insert_idx);
                self.tail = Some(insert_idx);
            } else {
                self.swap_to_most_recently_used(insert_idx);
            }
        } 
        // needs insertion, but over capacity; replace tail
        else {
            let replace_idx = self.tail.unwrap();

            let (old_tail_key, old_tail_prev) = {
                let node = &self.arena[replace_idx];
                (node.key, node.prev)
            };
            self.map.remove(&old_tail_key);

            self.map.insert(key, replace_idx);
            self.arena[replace_idx] = Node {
                key,
                value,
                next: None,
                prev: old_tail_prev,
            };
            self.swap_to_most_recently_used(replace_idx);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */