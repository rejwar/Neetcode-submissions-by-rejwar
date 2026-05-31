struct MyQueue {
    push_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.push_stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.move_elements();
        self.pop_stack.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        self.move_elements();
        *self.pop_stack.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.push_stack.is_empty() && self.pop_stack.is_empty()
    }
    
    fn move_elements(&mut self) {
        if self.pop_stack.is_empty() {
            while let Some(val) = self.push_stack.pop() {
                self.pop_stack.push(val);
            }
        }
    }
}