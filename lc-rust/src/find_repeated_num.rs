use std::collections::{HashMap, HashSet};
use crate::common_struct::Solution;

impl Solution {
    /// 在一个长度为 n 的数组 nums 里的所有数字都在 0～n-1 的范围内。数组中某些数字是重复的，
    /// 但不知道有几个数字重复了，也不知道每个数字重复了几次。请找出数组中任意一个重复的数字。
    /// 第一种方法
    /// 维持一个数组，a[n] 位置保存对应的数的个数
    /// 空间上有些浪费
    pub fn find_repeat_document(documents: Vec<usize>) -> usize {
        let length = documents.len();
        let mut store = vec![0;length];
        for num in documents {
            store[num] += 1;
            if store[num] >= 2 {
                return num
            }
        }
        panic!("there are no repeated number")
    }

    /// 使用hashmap来存储
    pub fn find_repeat_document_2(documents: Vec<usize>) -> usize {
        let mut store = HashMap::new();
        for num in documents {
            if store.contains_key(&num) {
                return num;
            } else {
                store.insert(num, 0);
            }
        }
        panic!("there are no repeated number")
    }

    /// 使用 HashSet 来存储
    ///
    pub fn find_repeat_document_3(documents: Vec<usize>) -> usize {
        let mut store = HashSet::new();
        for num in documents {
            if store.contains(&num) {
                return num;
            } else {
                store.insert(num);
            }
        }
        panic!("there are no repeated number")
    }
}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn ten_find_repeat_document() {
        assert_eq!(3, Solution::find_repeat_document(vec![1,3,3,1,5]));
    }

    #[test]
    fn ten_find_repeat_document_2() {
        assert_eq!(3, Solution::find_repeat_document_2(vec![1,3,3,1,1,5]));
        assert_eq!(7, Solution::find_repeat_document_2(vec![7,7,1,3,3,1,1,5]));
    }

    #[test]
    fn ten_find_repeat_document_3() {
        assert_eq!(0, Solution::find_repeat_document_3(vec![0,0]));
        assert_eq!(3, Solution::find_repeat_document_3(vec![1,3,3,1,1,5]));
        assert_eq!(7, Solution::find_repeat_document_3(vec![7,7,1,3,3,1,1,5]));
    }
}