/// 不同路径 II
/// 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
/// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
/// 现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？
/// 说明: 网格中的障碍物和空位置分别用 1 和 0 来表示。m 和 n 的值均不超过 100。

/// 示例
/// 输入:
/// [
///  [0,0,0],
///  [0,1,0],
///  [0,0,0]
/// ]
/// 输出: 2
/// 解释:
/// 3x3 网格的正中间有一个障碍物。
/// 从左上角到右下角一共有 2 条不同的路径：
/// 1. 向右 -> 向右 -> 向下 -> 向下
/// 2. 向下 -> 向下 -> 向右 -> 向右

pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() == 0 {
            return 0;
        }

        let (row, col) = (obstacle_grid.len(), obstacle_grid[0].len());

        // 使用一维数组代替二维数组，节省空间
        let mut paths = vec![0; col];

        // 起始点 0,0
        let (start_x, start_y) = (0, 0);
        // 第一个点初始化为 1
        paths[start_x] = 1;

        for i in start_y..row {
            for j in start_x..col {
                if obstacle_grid[i][j] == 1 {
                    paths[j] = 0;
                    continue;
                }
                if j as i32 - 1 >= 0 {
                    // paths 滑动数组
                    paths[j] += paths[j - 1];
                }
            }
        }
        // 返回最后的路径数目
        paths[col - 1]
    }
}

#[test]
fn test_unique_paths_with_obstacles() {
    let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let count = Solution::unique_paths_with_obstacles(obstacle_grid);
    println!("{}", count);
}
