mod solution;
mod merge;
use solution::Solution;

fn main() {
    let mut nums1 = vec![2, 0];
    let mut nums2 = vec![1];

    Solution::merge(&mut nums1, 1, &mut nums2, 1);
    for n in nums1 {
        print!("{} ", n);
    }
}
