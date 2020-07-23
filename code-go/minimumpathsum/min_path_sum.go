package main

import (
    "fmt"
    "math"
)

// 最小路径和
// 给定一个包含非负整数的 m x n 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
// 说明: 每次只能向下或者向右移动一步。

// 示例
// 输入:
// [
//   [1,3,1],
//   [1,5,1],
//   [4,2,1]
// ]
// 输出: 7
// 解释: 因为路径 1→3→1→1→1 的总和最小。

func minPathSum(grid [][]int) int {
    if len(grid) == 0 {
        return 0
    }
    row, col := len(grid), len(grid[0])
    // 用来保存原数组对应点的最小路径
    pathSums := make([][]int, row)
    for i := 0; i < len(pathSums); i++ {
        pathSums[i] = make([]int, col)
    }
    // 起始点 0,0
    startX, startY := 0, 0
    pathSums[startY][startX] = grid[startY][startX]

    // 第一列所有点的最小路径
    for i := startY + 1; i < row; i++ {
        pathSums[i][0] = pathSums[i - 1][0] + grid[i][0]
    }

    // 第一行所有点的最小路径
    for j := startX + 1; j < col; j++ {
        pathSums[0][j] = pathSums[0][j - 1] + grid[0][j]
    }

    // 其它点的最小路径，路径等于上边和左边中最小的值加上当前点的值
    for i := startY + 1; i < row; i++ {
        for j := startX + 1; j < col; j++ {
            pathSums[i][j] = int(math.Min(float64(pathSums[i - 1][j]),
                float64(pathSums[i][j - 1]))) + grid[i][j]
        }
    }
    // 返回最后一个点的最小路径
    return pathSums[row - 1][col - 1]
}


func main() {
    grid := [][]int {
        {1, 3, 1},
        {1, 5, 1},
        {4, 2, 1},
    }
    sum := minPathSum(grid)
    fmt.Println(sum)
}
