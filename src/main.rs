mod solution;
// mod merge;
// mod removeElement;
// mod removeDuplicate;
mod majorityElement;

use std::fmt::Display;
use solution::Solution;

fn displayVec<T: Display>(nums: &Vec<T>) {
    for c in nums.into_iter() {
        print!("{} ", c);
    }
}

fn main() {
    let mut nums = vec![1,2,3];
    let k = 4;
    Solution::rotate(&mut nums, k);
    displayVec(&nums);
}
