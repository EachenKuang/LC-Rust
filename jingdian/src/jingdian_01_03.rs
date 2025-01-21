// https://leetcode.cn/problems/string-to-url-lcci/description/?envType=study-plan-v2&envId=cracking-the-coding-interview
// URL化。编写一种方法，将字符串中的空格全部替换为%20。假定该字符串尾部有足够的空间存放新增字符，并且知道字符串的“真实”长度。（注：用Java实现的话，请使用字符数组实现，以便直接在数组上操作。）
//
// 示例 1：
//
// 输入："Mr John Smith    ", 13
// 输出："Mr%20John%20Smith"
// 示例 2：
//
// 输入："               ", 5
// 输出："%20%20%20%20%20"
//
// 提示：
//
// 字符串长度在 [0, 500000] 范围内。
use crate::common_struct::Solution;

impl Solution {
    /// 从后往前操作
    pub fn replace_spaces(s: String, length: i32) -> String {
        let mut result = String::new();
        for c in s.chars().take(length as usize) {
            if c == ' ' {
                result.push_str("%20");
            } else {
                result.push(c);
            }
        }
        result
    }

    /// 一行解决
    pub fn replace_spaces_2(s: String, length: i32) -> String {
        s[0..length as usize].replace(" ", "%20")
    }

}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_replace_spaces_1() {
        assert_eq!("%20", Solution::replace_spaces(String::from("   "), 1));
        assert_eq!("%20%20%20%20%20", Solution::replace_spaces(String::from("          "), 5));

    }

    #[test]
    fn test_replace_spaces_2() {
        assert_eq!("%20", Solution::replace_spaces_2(String::from("   "), 1));
        assert_eq!("%20%20%20%20%20", Solution::replace_spaces_2(String::from("          "), 5));
    }
}