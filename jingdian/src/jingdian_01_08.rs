// https://leetcode.cn/problems/zero-matrix-lcci/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 编写一种算法，若M × N矩阵中某个元素为0，则将其所在的行与列清零。
//
//
//
// 示例 1：
//
// 输入：
// [
//   [1,1,1],
//   [1,0,1],
//   [1,1,1]
// ]
// 输出：
// [
//   [1,0,1],
//   [0,0,0],
//   [1,0,1]
// ]
// 示例 2：
//
// 输入：
// [
//   [0,1,2,0],
//   [3,4,5,2],
//   [1,3,1,5]
// ]
// 输出：
// [
//   [0,0,0,0],
//   [0,4,5,0],
//   [0,3,1,0]
// ]
use crate::common_struct::Solution;

impl Solution {
    /// 遍历两次的方法
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut row_zero = vec![false; n];
        let mut col_zero = vec![false; m];
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    row_zero[i] = true;
                    col_zero[j] = true;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if row_zero[i] || col_zero[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    /// 不使用额外的空间进行存储
    /// 使用第一行和第一列作为标记是否需要置零，用两个变量存储是否需要设置第一行和第一列为0
    pub fn set_zeroes_2(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut first_row_zero = false;
        let mut first_col_zero = false;

        // 检查第一行和第一列是否需要置零
        for i in 0..n {
            if matrix[i][0] == 0 {
                first_col_zero = true;
                break;
            }
        }
        for j in 0..m {
            if matrix[0][j] == 0 {
                first_row_zero = true;
                break;
            }
        }

        // 利用第一行和第一列存储信息
        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        // 根据第一行和第一列的信息置零矩阵元素
        for i in 1..n {
            for j in 1..m {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // 置零第一行和第一列
        if first_row_zero {
            for j in 0..m {
                matrix[0][j] = 0;
            }
        }
        if first_col_zero {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    fn get_start_matrix() -> Vec<Vec<i32>> {
        vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]
        .clone()
    }

    fn get_result_matrix() -> Vec<Vec<i32>> {
        vec![
            vec![1, 0, 3, 4],
            vec![0, 0, 0, 0],
            vec![9, 0, 11, 12],
            vec![13, 0, 15, 16],
        ]
    }

    #[test]
    fn test_set_zeroes() {
        let mut matrix = get_start_matrix();
        Solution::set_zeroes(&mut matrix);
        println!("{:?}", matrix);
        assert_eq!(get_result_matrix(), matrix)
    }

    #[test]
    fn test_set_zeroes_2() {
        let mut matrix = get_start_matrix();
        Solution::set_zeroes_2(&mut matrix);
        println!("{:?}", matrix);
        assert_eq!(get_result_matrix(), matrix)
    }
}
