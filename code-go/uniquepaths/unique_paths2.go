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

    // 第一个点和最后一个点有障碍物
    if obstacleGrid[0][0] == 1 || obstacleGrid[row - 1][col - 1] == 1 {
        return 0
    }

    // 用来保存原数组从起点开始到对应点的路径数目
    pathSums := make([][]int, row)
    for i := 0; i < len(pathSums); i++ {
        pathSums[i] = make([]int, col)
    }
    // 起始点 0,0
    startX, startY := 0, 0
    // 起点路径数目为 1
    pathSums[startY][startX] = 1

    // 第一列所有点的路径数目
    for i := startY + 1; i < row; i++ {
        // 上一个点有障碍物时，不通，数目为0
        if obstacleGrid[i - 1][0] == 1 {
            pathSums[i][0] = 0
            continue
        }
        // 等于上一个点的路径数目
        pathSums[i][0] = pathSums[i - 1][0]
    }

    // 第一行所有点的路径数目
    for j := startX + 1; j < col; j++ {
        // 前一个点有障碍物时，不通，数目为0
        if obstacleGrid[0][j - 1] == 1 {
            pathSums[0][j] = 0
            continue
        }
        // 等于前一个点的路径数目
        pathSums[0][j] = pathSums[0][j - 1]
    }

    // 其它点的路径数目等于上边和左边点的路径数目之和
    for i := startY + 1; i < row; i++ {
        for j := startX + 1; j < col; j++ {
            // 上边和左边同时有障碍物，不通，数目为0
            if obstacleGrid[i - 1][j] == 1 && obstacleGrid[i][j - 1] == 1 {
                pathSums[i][j] = 0
                continue
            }
            // 上边有障碍物，时只能走左边
            if obstacleGrid[i - 1][j] == 1 {
                pathSums[i][j] = pathSums[i][j - 1]
                continue
            }
            // 左边有障碍物，时只能走右边
            if obstacleGrid[i][j - 1] == 1 {
                pathSums[i][j] = pathSums[i - 1][j]
                continue
            }
            pathSums[i][j] = pathSums[i - 1][j] + pathSums[i][j - 1]
        }
    }
    // 返回最后一个点的路径数目
    return pathSums[row - 1][col - 1]
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
