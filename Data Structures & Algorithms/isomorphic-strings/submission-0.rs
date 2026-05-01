impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map_s = [0; 256];
        let mut map_t = [0; 256];

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        for i in 0..s.len() {
            let cs = s_bytes[i] as usize;
            let ct = t_bytes[i] as usize;

            if map_s[cs] != map_t[ct] {
                return false;
            }

            map_s[cs] = i + 1;
            map_t[ct] = i + 1;
        }

        true
    }
}