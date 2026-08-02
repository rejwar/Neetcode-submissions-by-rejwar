struct BrowserHistory {
    history: Vec<String>,
    curr: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            curr: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.history.truncate(self.curr + 1);
        self.history.push(url);
        self.curr += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.curr = self.curr.saturating_sub(steps as usize);
        self.history[self.curr].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.curr = std::cmp::min(self.curr + (steps as usize), self.history.len() - 1);
        self.history[self.curr].clone()
    }
}