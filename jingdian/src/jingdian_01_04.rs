use std::collections::HashMap;
// https://leetcode.cn/problems/palindrome-permutation-lcci/description/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 给定一个字符串，编写一个函数判定其是否为某个回文串的排列之一。
//
// 回文串是指正反两个方向都一样的单词或短语。排列是指字母的重新排列。
//
// 回文串不一定是字典当中的单词。
//
//
//
// 示例1：
//
// 输入："tactcoa"
// 输出：true（排列有"tacocat"、"atcocta"，等等）


// 这道题与之前一道题有异曲同工之妙
use crate::common_struct::Solution;
impl Solution {

    /// 回文串的特性为：
    /// 长度为奇数 - count 字典 除了一个之外，都为偶数
    /// 长度为偶数 - count 字段 所有都为偶数
    pub fn can_permute_palindrome(s: String) -> bool {
        // 生成 count 字典
        let mut count = HashMap::new();
        for i in s.chars() {
            count.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }
        let odd_count = count.values().filter(|&&e| e%2 == 1).count();
        odd_count <= 1
    }

    /// 使用一个 u128 位 无符号数记录，因为只需要知道 对应的数量是奇数还是偶数，所以可以使用异或运算执行
    /// 此方法利用了 u128 的 128 位来存储信息，适用于处理字符范围在 u128 能表示的范围内的情况。对于大部分的字符串，
    /// 特别是只包含 ASCII 字符的字符串，这个范围足够。但对于包含更广泛字符集的字符串，需要考虑更大的位集或其他方法。
    /// 这种实现方式在空间复杂度上相对优化，因为使用了固定大小的 u128 来存储信息，避免了使用 HashMap 等可能需要更多空间的数据结构。
    pub fn can_permute_palindrome_2(s: String) -> bool {
        let mut bitmap: u128 = 0;
        for ch in s.chars() {
            bitmap ^= 1 << (ch as u128)
        }
        bitmap & (bitmap - 1) == 0 // 检查 bitmap 中是否最多只有一个位为 1，以判断字符串是否是某个回文串的排列之一。
    }
}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_can_permute_palindrome() {
        assert_eq!(Solution::can_permute_palindrome("code".to_string()), false);
        assert_eq!(Solution::can_permute_palindrome("tactcoa".to_string()), true);
    }

    #[test]
    fn test_can_permute_palindrome_2() {
        assert_eq!(Solution::can_permute_palindrome_2("code".to_string()), false);
        assert_eq!(Solution::can_permute_palindrome_2("tactcoa".to_string()), true);
    }
}