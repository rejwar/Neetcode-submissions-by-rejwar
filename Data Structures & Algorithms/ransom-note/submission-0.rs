impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut counts = [0; 26];
        
        for b in magazine.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        
        for b in ransom_note.bytes() {
            let idx = (b - b'a') as usize;
            if counts[idx] == 0 {
                return false;
            }
            counts[idx] -= 1;
        }
        
        true
    }
}