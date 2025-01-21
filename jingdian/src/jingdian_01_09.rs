use crate::common_struct::Solution;

// https://leetcode.cn/problems/string-rotation-lcci/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 字符串轮转。给定两个字符串s1和s2，请编写代码检查s2是否为s1旋转而成（比如，waterbottle是erbottlewat旋转后的字符串）。
//
// 示例 1：
//
//  输入：s1 = "waterbottle", s2 = "erbottlewat"
//  输出：True
// 示例 2：
//
//  输入：s1 = "aa", s2 = "aba"
//  输出：False
// 提示：
//
// 字符串长度在[0, 100000]范围内。
// 说明:
//
// 你能只调用一次检查子串的方法吗？

impl Solution {
    /// If one string A can flip into another string B, it must have a rule for A and B:
    /// String BB contains A
    /// eg: A:abcd B:cdab  BB:cd[abcd]ab
    pub fn is_flipped_string(s1: String, s2: String) -> bool {
        let length_s1 = s1.len();
        let length_s2 = s2.len();
        if length_s1 != length_s2 {
            return false;
        }
        let combine = s2.clone() + &s2;
        combine.contains(&s1)
    }
}

#[cfg(test)]
mod test {
    use crate::common_struct::Solution;

    #[test]
    fn test_is_flipped_string() {
        assert_eq!(
            true,
            Solution::is_flipped_string("waterbottle".to_string(), "erbottlewat".to_string())
        );
        assert_eq!(
            false,
            Solution::is_flipped_string("aa".to_string(), "ab".to_string())
        );
    }
}
