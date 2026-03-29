use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    small: BinaryHeap<i32>,           
    large: BinaryHeap<Reverse<i32>>,  
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        
        self.small.push(num);

        
        if let (Some(&s), Some(&Reverse(l))) = (self.small.peek(), self.large.peek()) {
            if s > l {
                let val = self.small.pop().unwrap();
                self.large.push(Reverse(val));
            }
        }

  
        if self.small.len() > self.large.len() + 1 {
            let val = self.small.pop().unwrap();
            self.large.push(Reverse(val));
        } else if self.large.len() > self.small.len() + 1 {
            if let Some(Reverse(val)) = self.large.pop() {
                self.small.push(val);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            return *self.small.peek().unwrap() as f64;
        } else if self.large.len() > self.small.len() {
            let Reverse(val) = *self.large.peek().unwrap();
            return val as f64;
        }
        
        let s = *self.small.peek().unwrap() as f64;
        let Reverse(l) = *self.large.peek().unwrap();
        (s + l as f64) / 2.0
    }
}