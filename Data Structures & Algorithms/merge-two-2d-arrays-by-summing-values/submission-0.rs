impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let (mut i, mut j) = (0, 0);

        while i < nums1.len() && j < nums2.len() {
            if nums1[i][0] == nums2[j][0] {
                res.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                i += 1;
                j += 1;
            } else if nums1[i][0] < nums2[j][0] {
                res.push(nums1[i].clone());
                i += 1;
            } else {
                res.push(nums2[j].clone());
                j += 1;
            }
        }

        while i < nums1.len() {
            res.push(nums1[i].clone());
            i += 1;
        }

        while j < nums2.len() {
            res.push(nums2[j].clone());
            j += 1;
        }

        res
    }
}