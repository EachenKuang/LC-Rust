// https://leetcode.cn/problems/compress-string-lcci/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 字符串压缩。利用字符重复出现的次数，编写一种方法，实现基本的字符串压缩功能。比如，字符串aabcccccaaa会变为a2b1c5a3。若“压缩”后的字符串没有变短，则返回原先的字符串。你可以假设字符串中只包含大小写英文字母（a至z）。
//
// 示例 1：
//
// 输入："aabcccccaaa"
// 输出："a2b1c5a3"
// 示例 2：
//
// 输入："abbccd"
// 输出："abbccd"
// 解释："abbccd"压缩后为"a1b2c2d1"，比原字符串长度更长。
// 提示：
//
// 字符串长度在 [0, 50000] 范围内。
use crate::common_struct::Solution;

impl Solution {
    pub fn compress_string(s: String) -> String {
        let mut result = String::new();
        let mut current_char = ' ';
        let mut current_char_count = 0;
        for c in s.chars() {
            if c == current_char {
                current_char_count += 1;
            } else {
                if current_char_count > 0 {
                    result.push(current_char);
                    result.push_str(current_char_count.to_string().as_str());
                }
                current_char = c;
                current_char_count = 1
            }
        }
        if current_char_count > 0 {
            // 处理末尾
            result.push(current_char);
            result.push_str(current_char_count.to_string().as_str());
        }

        return if result.len() < s.len() {
            result
        } else {
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_compress_string() {
        assert_eq!("a2b1c5a3".to_string(), Solution::compress_string("aabcccccaaa".to_string()));
        assert_eq!("abbccd", Solution::compress_string("abbccd".to_string()));
    }
}