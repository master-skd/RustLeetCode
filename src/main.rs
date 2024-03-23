mod solution;
// mod merge;
// mod removeElement;
mod removeDuplicate;

use std::fmt::Display;
use solution::Solution;

fn diplayVec<T: Display>(nums: &Vec<T>) {
    for c in nums.into_iter() {
        print!("{} ", c);
    }
}

fn main() {
    let mut nums = vec![0,0,1,1,1,1,2,3,3];
    let res = Solution::remove_duplicates(&mut nums);
    println!("{}", res);
    diplayVec(&nums);
}
