struct MyHashSet {
    set: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            set: vec![false; 1_000_001],
        }
    }
    
    fn add(&mut self, key: i32) {
        self.set[key as usize] = true;
    }
    
    fn remove(&mut self, key: i32) {
        self.set[key as usize] = false;
    }
    
    fn contains(&self, key: i32) -> bool {
        self.set[key as usize]
    }
}