struct MyHashMap {
    map: Vec<i32>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            map: vec![-1; 1_000_001],
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.map[key as usize] = value;
    }
    
    fn get(&self, key: i32) -> i32 {
        self.map[key as usize]
    }
    
    fn remove(&mut self, key: i32) {
        self.map[key as usize] = -1;
    }
}