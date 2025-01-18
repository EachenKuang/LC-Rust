use std::collections::HashMap;

pub struct Solution;

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map = HashMap::new();

        for (id, val) in nums.iter().enumerate() {
            if let Some(val) = map.get(&(target - val)) {
                return vec![id as i32, *val as i32]
            } else {
                map.insert(val, id);
            }
        }
        panic!();
    }


    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&idx) = map.get(&num) {
                return vec![idx as i32, i as i32];
            }
            map.insert(target - num, i);
        }
        vec![]
    }
}
