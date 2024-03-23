use std::collections::HashMap;
use super::solution::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let threshold = nums.len() >> 1;
        let mut res: HashMap<i32, usize> = HashMap::new();  // 构建一个哈希表，存储每个元素出现的个数
        for num in &nums {
            match res.get(num) {
                Some(v) => res.insert(*num, v + 1),
                None => res.insert(*num, 1)
            };
        }
        for (num, cnt) in res.iter() {
            if *cnt > threshold {
                return *num;
            }
        }
        return 0;
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {

    }
}
