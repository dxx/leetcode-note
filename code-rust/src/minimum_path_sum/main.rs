/// 最小路径和
/// 给定一个包含非负整数的 m x n 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
/// 说明: 每次只能向下或者向右移动一步。

/// 示例
/// 输入:
/// [
///   [1,3,1],
///   [1,5,1],
///   [4,2,1]
/// ]
/// 输出: 7
/// 解释: 因为路径 1→3→1→1→1 的总和最小。

pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let (row, col) = (grid.len(), grid[0].len());
        // 用来保存原数组对应点的最小路径
        let mut path_sums = vec![vec![0; col]; row];

        // 起始点 0,0
        let (start_x, start_y) = (0, 0);
        path_sums[start_y][start_x] = grid[start_y][start_x];

        // 第一列所有点的最小路径
        for i in start_y + 1..row {
            path_sums[i][0] = path_sums[i - 1][0] + grid[i][0];
        }

        // 第一行所有点的最小路径
        for j in start_x + 1..col {
            path_sums[0][j] = path_sums[0][j - 1] + grid[0][j];
        }

        // 其它点的最小路径，路径等于上边和左边中最小的值加上当前点的值
        for i in start_y + 1..row {
            for j in start_x + 1..col {
                path_sums[i][j] =
                    std::cmp::min(path_sums[i - 1][j], path_sums[i][j - 1]) + grid[i][j];
            }
        }

        // 返回最后一个点的最小路径
        path_sums[row - 1][col - 1]
    }
}

#[test]
fn test_min_path_sum() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    let sum = Solution::min_path_sum(grid);
    println!("{}", sum);
}
