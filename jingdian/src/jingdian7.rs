// https://leetcode.cn/problems/rotate-matrix-lcci/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 给你一幅由 N × N 矩阵表示的图像，其中每个像素的大小为 4 字节。请你设计一种算法，将图像旋转 90 度。
//
// 不占用额外内存空间能否做到？
//
//
//
// 示例 1：
//
// 给定 matrix =
// [
//   [1,2,3],
//   [4,5,6],
//   [7,8,9]
// ],
//
// 原地旋转输入矩阵，使其变为:
// [
//   [7,4,1],
//   [8,5,2],
//   [9,6,3]
// ]
// 示例 2：
//
// 给定 matrix =
// [
//   [ 5, 1, 9,11],
//   [ 2, 4, 8,10],
//   [13, 3, 6, 7],
//   [15,14,12,16]
// ],
//
// 原地旋转输入矩阵，使其变为:
// [
//   [15,13, 2, 5],
//   [14, 3, 4, 1],
//   [12, 6, 8, 9],
//   [16, 7,10,11]
// ]
use crate::common_struct::Solution;

impl Solution {
    ///
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        // 先转置
        for i in 0..n {
            for j in i..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        // 再翻转
        for i in 0..n {
            for j in 0..n/2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[i][n - 1 - j];
                matrix[i][n - 1 - j] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_struct::Solution;

    #[test]
    fn test_rotate() {
        let mut matrix: Vec<Vec<i32>> = vec![
            vec![1,2,3,4],
            vec![5,6,7,8],
            vec![9,10,11,12],
            vec![13,14,15,16],
        ];

        Solution::rotate(&mut matrix);

        println!("{:?}", matrix);

        // assert_eq!("a2b1c5a3".to_string(), Solution::rotate("aabcccccaaa".to_string()));
    }
}