// 在一个 n * m 的二维数组中，每一行都按照从左到右递增的顺序排序，每一列都按照从上到下递增的顺序排序。请完成一个高效的函数，
// 输入这样的一个二维数组和一个整数，判断数组中是否含有该整数。
// #
// 示例:
// #
// 现有矩阵 matrix 如下：
// [
//   [1,   4,  7, 11, 15],
//   [2,   5,  8, 12, 19],
//   [3,   6,  9, 16, 22],
//   [10, 13, 14, 17, 24],
//   [18, 21, 23, 26, 30]
// ]
// 给定target=5，返回true。
// #
// 给定target=20，返回false。
// #
// 限制：
// 0 <= n <= 1000
// 0 <= m <= 1000
use crate::common_struct::Solution;

impl Solution {

    /// 全部遍历
    pub fn find_number_in_matrix(matrix: &[Vec<i32>], target: i32) -> bool{
        let rows = matrix.len();
        if rows == 0 {
            return false;
        }
        let cols = matrix[0].len();
        let mut row = 0;
        let mut col = cols - 1;
        while row < rows {
            if matrix[row][col] == target {
                return true;
            } else if matrix[row][col] > target {
                if col == 0 {
                    // 避免 usize 溢出
                    break
                }
                col -= 1;
            } else {
                row += 1;
            }
        }
        false
    }

}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_find_number_in_matrix() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert_eq!(true, Solution::find_number_in_matrix(&matrix, 1));
        assert_eq!(false, Solution::find_number_in_matrix(&matrix, 0));
    }
}