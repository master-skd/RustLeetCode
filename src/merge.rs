use super::solution::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p = m - 1;
        let mut q = n - 1;
        let mut k = m + n - 1;

        while k >= 0 {
            if p < 0 {  // 说明此时nums2中的元素均是小元素
                nums1[k as usize] = nums2[q as usize];
                q -= 1;
            } else if q < 0 {  // 说明此时nums1中的元素均是小元素
                nums1[k as usize] = nums1[p as usize];
                p -= 1;
            } else if nums1[p as usize] < nums2[q as usize] {
                nums1[k as usize] = nums2[q as usize];
                q -= 1;
            } else {
                nums1[k as usize] = nums1[p as usize];
                p -= 1;
            }
            k -= 1;
        }
    }
}
