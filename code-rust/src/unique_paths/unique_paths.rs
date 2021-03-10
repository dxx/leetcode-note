/// 不同路径
/// 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
/// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
/// 问总共有多少条不同的路径？

/// 示例
/// 输入: m = 3, n = 2
/// 输出: 3
/// 解释:
/// 从左上角开始，总共有 3 条路径可以到达右下角。
/// 1. 向右 -> 向右 -> 向下
/// 2. 向右 -> 向下 -> 向右
/// 3. 向下 -> 向右 -> 向右

/// 输入: m = 7, n = 3
/// 输出: 28

pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // let (row, col) = (n as usize, m as usize);
        // // 用来保存原数组从起点开始到对应点的路径数目
        // let mut paths = vec![vec![0; col]; row];
        //
        // // 起始点 0,0
        // let (start_x, start_y) = (0, 0);
        //
        // // 第一列所有点的路径数目
        // for i in start_y..row {
        //     paths[i][0] = 1
        // }
        //
        // // 第一行所有点的路径数目
        // for j in start_x..col {
        //     paths[0][j] = 1
        // }
        //
        // // 其它点的路径数目等于上边和左边点的路径数目之和
        // for i in start_y + 1..row {
        //     for j in start_x + 1..col {
        //         paths[i][j] = paths[i - 1][j] + paths[i][j - 1]
        //     }
        // }
        // // 返回最后一个点的路径数目
        // paths[row - 1][col - 1]

        let (row, col) = (n as usize, m as usize);

        // 使用一维数组代替二维数组，节省空间
        let mut paths = vec![1; col];

        // 起始点 0,0
        let (start_x, start_y) = (0, 0);

        for _ in start_y + 1..row {
            for j in start_x + 1..col {
                // paths 滑动数组
                paths[j] += paths[j - 1]
            }
        }
        // 返回最后的路径数目
        paths[col - 1]
    }
}

#[test]
fn test_unique_paths() {
    let m = 3;
    let n = 2;
    println!("m = {}, n = {}", m, n);
    let count = Solution::unique_paths(m, n);
    println!("{}", count);

    let m = 7;
    let n = 3;
    println!("m = {}, n = {}", m, n);
    let count = Solution::unique_paths(m, n);
    println!("{}", count);
}
