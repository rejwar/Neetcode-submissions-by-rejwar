struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        while let Some(val) = self.s1.pop() {
            self.s2.push(val);
        }
        self.s1.push(x);
        while let Some(val) = self.s2.pop() {
            self.s1.push(val);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.s1.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        *self.s1.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.s1.is_empty()
    }
}