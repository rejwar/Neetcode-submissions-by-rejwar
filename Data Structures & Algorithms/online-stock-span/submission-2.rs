struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    
    pub fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        
        while let Some(&(prev_price, prev_span)) = self.stack.last() {
            if prev_price <= price {
                span += prev_span;
                self.stack.pop();
            } else {
                break;
            }
        }
        
        self.stack.push((price, span));
        span
    }
}