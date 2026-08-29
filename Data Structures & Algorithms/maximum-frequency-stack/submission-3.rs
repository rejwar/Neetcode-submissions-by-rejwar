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
        // Increment the frequency for the value
        let f = self.freq.entry(val).or_insert(0);
        *f += 1;
        
        // Update the maximum frequency if necessary
        if *f > self.max_freq {
            self.max_freq = *f;
        }
        
     
        self.group.entry(*f).or_default().push(val);
    }
    
    pub fn pop(&mut self) -> i32 {
      
        let stack = self.group.get_mut(&self.max_freq).unwrap();
        
       
        let val = stack.pop().unwrap();
        
      
        *self.freq.get_mut(&val).unwrap() -= 1;
      
        if stack.is_empty() {
            self.max_freq -= 1;
        }
        
        val
    }
}