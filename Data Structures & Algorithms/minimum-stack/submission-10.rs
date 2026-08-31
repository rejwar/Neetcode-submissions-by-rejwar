struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }
    
    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        // Push onto min_stack if it's empty or the new value is a new minimum
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }
    
    pub fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            // If the popped value is the current minimum, remove it from min_stack as well
            if val == *self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
        }
    }
    
    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}