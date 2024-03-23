use super::solution::Solution;

impl Solution {
    pub fn remove_duplicate(nums: &mut Vec<i32>) -> i32 {
        let mut p: usize = 0;
        let mut q: usize = 1;
        let len = nums.len();
        while q < len {
            if nums[q] != nums[p] {
                p += 1;
                nums[p] = nums[q];
            }
            q += 1;
        }
        p += 1;
        return p as i32;
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut p: usize = 0;
        let mut q: usize = 1;
        let mut count = 1;
        let len = nums.len();
        while q < len {
            if nums[q] == nums[p] && count < 2 {
                p += 1;
                nums[p] = nums[q];
                count += 1;
            }
            if nums[q] != nums[p]{
                p += 1;
                nums[p] = nums[q];
                count = 1;
            }
            q += 1;
        }
        p += 1;
        return p as i32;
    }
}
