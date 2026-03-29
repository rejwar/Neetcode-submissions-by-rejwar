impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let word_chars: Vec<char> = word.chars().collect();

        fn backtrack(
            r: i32,
            c: i32,
            i: usize,
            board: &mut Vec<Vec<char>>,
            word: &[char],
        ) -> bool {
            if i == word.len() {
                return true;
            }
            if r < 0 || r >= board.len() as i32 || 
               c < 0 || c >= board[0].len() as i32 || 
               board[r as usize][c as usize] != word[i] {
                return false;
            }

            let temp = board[r as usize][c as usize];
            board[r as usize][c as usize] = '#'; // Mark as visited

            let res = backtrack(r + 1, c, i + 1, board, word) ||
                      backtrack(r - 1, c, i + 1, board, word) ||
                      backtrack(r, c + 1, i + 1, board, word) ||
                      backtrack(r, c - 1, i + 1, board, word);

            board[r as usize][c as usize] = temp; // Backtrack
            res
        }

        for r in 0..rows {
            for c in 0..cols {
                if backtrack(r as i32, c as i32, 0, &mut board, &word_chars) {
                    return true;
                }
            }
        }
        false
    }}