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
    let nums = vec![2,2,1,1,1,2,2];
    let res = Solution::majority_element(nums);
    println!("{}", res);
}
