mod solution;
// mod merge;
// mod removeElement;
// mod removeDuplicate;
// mod majorityElement;
mod jump;

use std::fmt::Display;
use solution::Solution;

fn displayVec<T: Display>(nums: &Vec<T>) {
    for c in nums.into_iter() {
        print!("{} ", c);
    }
}

fn main() {
    let mut nums = vec![7,1,5,3,6,4];
    let res = Solution::max_profit2(nums);
    println!("{}", res);
}
