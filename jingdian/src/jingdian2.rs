use std::collections::HashMap;
// https://leetcode.cn/problems/check-permutation-lcci/description/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 给定两个由小写字母组成的字符串 s1 和 s2，请编写一个程序，确定其中一个字符串的字符重新排列后，能否变成另一个字符串。
//
// 示例 1：
//
// 输入: s1 = "abc", s2 = "bca"
// 输出: true
// 示例 2：
//
// 输入: s1 = "abc", s2 = "bad"
// 输出: false
// 说明：use crate::common_struct::Solution;
//
// 0 <= len(s1) <= 100
// 0 <= len(s2) <= 100
use crate::common_struct::Solution;


impl Solution {
    /// 排序后进行比较
    ///
    pub fn check_permutation_0(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let (mut s1, mut s2) = (Vec::from(s1), Vec::from(s2));
        s1.sort();
        s2.sort();
        s1 == s2
    }

    /// 重排列，说明组成键值对是能够完全一致，key位字符，value位字符个个数
    /// 使用 hashmap进行数据存储
    pub fn check_permutation_1(s1: String, s2: String) -> bool {
        // 剪枝
        if s1.len() != s2.len() {
            return false;
        }
        let mut hash = HashMap::new();
        for i in s1.chars() {
            if let Some(value) = hash.get_mut(&i) {
                *value += 1;
            } else {
                hash.insert(i, 1);
            }
        }
        for j in s2.chars() {
            if let Some(value) = hash.get_mut(&j) {
                if *value == 0 {
                    // 提前返回，如果一个key的值小于0，说明已经不一致了
                    return false
                }
                *value -= 1;
            } else {
                // 如果有不一致的key，直接返回
                return false;
            }
        }

        // 验证 hash 是否都是0
        for i in hash.values() {
            if *i != 0 {
                return false
            }
        }

        true
    }
    /// 改进：使用数组进行存储
    pub fn check_permutation_2(s1: String, s2: String) -> bool {
        // 剪枝
        if s1.len() != s2.len() {
            return false;
        }
        let mut array = [0;26];
        for i in s1.chars() {
            array[i as usize - 'a' as usize] += 1;
        }
        for j in s2.chars() {
            if array[j as usize - 'a' as usize] == 0 {
                return false;
            } else {
                array[j as usize - 'a' as usize] -= 1;
            }
        }

        // 验证 hash 是否都是0
        // 也可以使用 array.iter().all(|x| *x == 0)
        for i in array {
            if i != 0 {
                return false
            }
        }

        true
    }

    /// 使用库函数执行，内部count方法计算数量，不推荐
    pub fn check_permutation_3(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() { return false; }
        s1.chars().all(|c| s1.matches(c).count() == s2.matches(c).count())
    }
}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_check_permutation_0() {
        assert_eq!(false, Solution::check_permutation_0("abc".to_string(), "bad".to_string()));
        assert_eq!(true, Solution::check_permutation_0("abc".to_string(), "cba".to_string()));
    }

    #[test]
    fn test_check_permutation_1() {
        assert_eq!(false, Solution::check_permutation_1("abc".to_string(), "bad".to_string()));
        assert_eq!(true, Solution::check_permutation_1("abc".to_string(), "cba".to_string()));
    }

    #[test]
    fn test_check_permutation_2() {
        assert_eq!(false, Solution::check_permutation_2("abc".to_string(), "bad".to_string()));
        assert_eq!(true, Solution::check_permutation_2("abc".to_string(), "cba".to_string()));
    }

    #[test]
    fn test_check_permutation_3() {
        assert_eq!(false, Solution::check_permutation_3("abc".to_string(), "bad".to_string()));
        assert_eq!(true, Solution::check_permutation_3("abc".to_string(), "cba".to_string()));
    }
}