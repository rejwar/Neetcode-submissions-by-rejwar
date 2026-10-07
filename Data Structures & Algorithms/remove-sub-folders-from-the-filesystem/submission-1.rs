impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut res: Vec<String> = Vec::new();
        
        for f in folder {
            if let Some(last) = res.last() {
                if f.starts_with(last) && f.as_bytes().get(last.len()) == Some(&b'/') {
                    continue;
                }
            }
            res.push(f);
        }
        
        res
    }
}