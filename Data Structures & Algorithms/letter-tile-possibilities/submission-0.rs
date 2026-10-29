impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts = [0; 26];
        for b in tiles.bytes() {
            counts[(b - b'A') as usize] += 1;
        }
        Self::backtrack(&mut counts)
    }

    fn backtrack(counts: &mut [i32; 26]) -> i32 {
        let mut total = 0;
        for i in 0..26 {
            if counts[i] > 0 {
                total += 1;
                counts[i] -= 1;
                total += Self::backtrack(counts);
                counts[i] += 1;
            }
        }
        total
    }
}