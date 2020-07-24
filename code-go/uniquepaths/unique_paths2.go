package main

import "fmt"

// 不同路径 II
// 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
// 现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？
// 说明: 网格中的障碍物和空位置分别用 1 和 0 来表示。m 和 n 的值均不超过 100。

// 示例
// 输入:
// [
//   [0,0,0],
//   [0,1,0],
//   [0,0,0]
// ]
// 输出: 2
// 解释:
// 3x3 网格的正中间有一个障碍物。
// 从左上角到右下角一共有 2 条不同的路径：
// 1. 向右 -> 向右 -> 向下 -> 向下
// 2. 向下 -> 向下 -> 向右 -> 向右

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
    if len(obstacleGrid) == 0 {
        return 0
    }

    row, col := len(obstacleGrid), len(obstacleGrid[0])

    // 使用一维数组代替二维数组，节省空间
    paths := make([]int, col)

    // 起始点 0,0
    startX, startY := 0, 0
    // 第一个点初始化为 1
    paths[startX] = 1

    for i := startY; i < row; i++ {
        for j := startX; j < col; j++ {
            // 有障碍物，当前点的路径数目为 0
            if obstacleGrid[i][j] == 1 {
                paths[j] = 0
                continue
            }
            if j - 1 >= 0 {
                // paths 滑动数组
                paths[j] += paths[j - 1]
            }
        }
    }
    // 返回最后的路径数目
    return paths[col - 1]
}

func main() {
    obstacleGrid := [][]int {
        {0, 0, 0},
        {0, 1, 0},
        {0, 0, 0},
    }
    count := uniquePathsWithObstacles(obstacleGrid)
    fmt.Println(count)
}
