struct MinStack {
    // 
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    
    pub fn push(&mut self, val: i32) {
        let current_min = match self.stack.last() {
            Some(&(_, min_so_far)) => val.min(min_so_far),
            None => val,
        };
        self.stack.push((val, current_min));
    }
    
    pub fn pop(&mut self) {
        self.stack.pop();
    }
    
    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}