// 字符串有三种编辑操作:插入一个英文字符、删除一个英文字符或者替换一个英文字符。 给定两个字符串，编写一个函数判定它们是否只需要一次(或者零次)编辑。
//
//
//
// 示例 1：
//
// 输入：
// first = "pale"
// second = "ple"
// 输出：True
//
//
// 示例 2：
//
// 输入：
// first = "pales"
// second = "pal"
// 输出：False
use crate::common_struct::Solution;

impl Solution {

    /// 基础思路解题
    /// 剪枝，判断长度是否相差小于1，大于一以上就可以直接返回 false
    /// 如果两个相同长度，只能有一个位置的字符不一样，使用替换原则即可
    /// 如果两个字符串不一样长，可以通过对于短的增加字符或者长的删除字符
    pub fn one_edit_away(first: String, second: String) -> bool {
        let length_first = first.len();
        let length_second = second.len();

        let len_diff = (length_first as i32 - length_second as i32).abs();

        if len_diff > 1 {
            return false;
        }

        let mut i = 0;
        let mut j = 0;
        let mut edit_count = 0;

        while i < length_first && j < length_second {
            if first.chars().nth(i) == second.chars().nth(j) {
                i += 1;
                j += 1;
            } else {
                if length_first == length_second {
                    // 长度相同，需要替换
                    edit_count += 1;
                    i += 1;
                    j += 1;
                } else if length_first > length_second {
                    // 需要删除
                    i += 1;
                    edit_count += 1;
                } else {
                    // 需要添加
                    j += 1;
                    edit_count += 1;
                }
                // 如果执行修改数量超过1
                if edit_count > 1 {
                    return false;
                }
            }


            i += 1;
            j += 1;
        }
        // 处理剩余的字符
        if i < length_first || j < length_second {
            edit_count += 1;
        }
        edit_count <= 1
    }


    /// 动态规划解题 【通用方法】
    /// 此题不一定需要动态规划来解，因为只是许哟啊确定是否，如果需要计算最小的编辑距离，使用动态规划会更快捷
    /// 使用一个二维数组 dp[i][j] 表示将 first 的前 i 个字符转换成 second 的前 j 个字符所需的最少编辑操作次数。
    pub fn one_edit_away_2(first: String, second: String) -> bool {
        let m = first.len();
        let n = second.len();
        // 处理长度差大于 1 的情况
        if (m as i32 - n as i32).abs() > 1 {
            return false;
        }
        let mut dp = vec![vec![0; n + 1]; m + 1];
        // 初始化第一行和第一列
        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }
        for i in 1..=m {
            for j in 1..=n {
                if first.chars().nth(i - 1) == second.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    //
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
            }
        }
        dp[m][n] <= 1
    }
}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_one_edit_away_1() {
        assert_eq!(false, Solution::one_edit_away("abc".to_string(), "bad".to_string()));
        assert_eq!(true, Solution::one_edit_away("aba".to_string(), "cba".to_string()));
    }

    #[test]
    fn test_one_edit_away_2() {
        assert_eq!(false, Solution::one_edit_away_2("abc".to_string(), "bad".to_string()));
        assert_eq!(true, Solution::one_edit_away_2("aba".to_string(), "cba".to_string()));
    }
}