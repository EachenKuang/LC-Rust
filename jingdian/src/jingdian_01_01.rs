// https://leetcode.cn/problems/is-unique-lcci/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 实现一个算法，确定一个字符串 s 的所有字符是否全都不同。
//
// 示例 1：
//
// 输入: s = "leetcode"
// 输出: false
// 示例 2：
//
// 输入: s = "abc"
// 输出: true
// 限制：
//
// 0 <= len(s) <= 100
// s[i]仅包含小写字母
// 如果你不使用额外的数据结构，会很加分。
use crate::common_struct::Solution;


impl Solution {
    /// 因为只有小写字母，所以可以使用一个数组保存
    pub fn is_unique(astr: String) -> bool {
        // 剪枝，快速返回，如果长度超过26，那么一定有相同的字符
        if astr.len() > 26 {
            return false;
        }
        let mut array = [0; 26];  // 设置一个26长度的数组用于存储对应的小写字母的个数
        for s in astr.chars() {
            let index = s as usize - 'a' as usize;
            if array[index] == 0 {
                array[index] = 1;
            } else {
                return false;
            }
        }
        true
    }

    /// 使因为小写字母只有26个，使用一个 int32位的 整数可以用26位来存储小写字母是否存在的情况
    ///
    pub fn is_unique_2(astr: String) -> bool {
        // 剪枝，快速返回，如果长度超过26，那么一定有相同的字符
        if astr.len() > 26 {
            return false;
        }
        let mut bit_map = 0;
        for s in astr.chars() {
            let index = s as usize - 'a' as usize;
            if bit_map | 1 << index == bit_map {
                return false;
            } else {
                bit_map |= 1 << index;  // 将 bit_map 置位
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_is_unique() {
        assert_eq!(false, Solution::is_unique(String::from("leetcode")));
        assert_eq!(true, Solution::is_unique(String::from("abc")));
    }

    #[test]
    fn test_is_unique_2() {
        assert_eq!(false, Solution::is_unique_2(String::from("leetcode")));
        assert_eq!(true, Solution::is_unique_2(String::from("abc")));
    }
}