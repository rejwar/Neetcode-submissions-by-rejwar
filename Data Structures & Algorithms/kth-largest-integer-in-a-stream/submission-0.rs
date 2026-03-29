use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut obj = KthLargest {
            min_heap: BinaryHeap::new(),
            k: k as usize,
        };
        for num in nums {
            obj.add(num);
        }
        obj
    }

    fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));
        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }
        self.min_heap.peek().unwrap().0
    }
}