use super::solution::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut p: usize = 0;
        let mut q: usize = 0;
        let mut res: i32 = 0;
        let len = nums.len();
        while p < len {
            if nums[p] != val {
                nums[q] = nums[p];
                q += 1;
                res += 1;
            }
            p += 1;
        }
        return res;
    }
}
