use std::collections::HashMap;

struct FreqStack {
    freq: HashMap<i32, i32>,
    group: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

impl FreqStack {
    pub fn new() -> Self {
        FreqStack {
            freq: HashMap::new(),
            group: HashMap::new(),
            max_freq: 0,
        }
    }

    pub fn push(&mut self, val: i32) {
        let count = self.freq.entry(val).or_insert(0);
        *count += 1;
        let c_freq = *count;

        if c_freq > self.max_freq {
            self.max_freq = c_freq;
        }

        self.group.entry(c_freq).or_insert(Vec::new()).push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let stack = self.group.get_mut(&self.max_freq).unwrap();
        let val = stack.pop().unwrap();

        if let Some(count) = self.freq.get_mut(&val) {
            *count -= 1;
        }

        if stack.is_empty() {
            self.max_freq -= 1;
        }

        val
    }
}