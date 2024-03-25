use std::cmp::min;
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

    pub fn reverse(nums: &mut Vec<i32>, begin: i32, end: i32) {
        let mut i = begin;
        let mut j = end;
        while i <= j {
            let temp = nums[i as usize];
            nums[i as usize] = nums[j as usize];
            nums[j as usize] = temp;
            i += 1;
            j -= 1;
        }
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let rk = k % length as i32;
        Solution::reverse(nums, 0, length as i32 - rk - 1);
        Solution::reverse(nums, length as i32 - rk, length as i32 - 1);
        Solution::reverse(nums, 0, length as i32 - 1);
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut minprice: i32 = 100000;
        let mut maxprofit = 0;
        for price in prices {
            maxprofit = if maxprofit > price - minprice {
                maxprofit
            } else {
                price - minprice
            };
            minprice = if minprice < price {
                minprice
            } else {
                price
            };
        }
        return maxprofit;
    }

    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut maxprofit: i32 = 0;
        for i in 1..len {
            if prices[i - 1] < prices[i] {
                maxprofit += prices[i] - prices[i - 1];
            }
        }
        return maxprofit;
    }
}
