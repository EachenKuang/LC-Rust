mod _1_two_sum;
mod _2_add_two_numbers;
use crate::_1_two_sum::Solution;
use crate::_2_add_two_numbers::Solution;

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
    assert_eq!(Solution::two_sum_2(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
