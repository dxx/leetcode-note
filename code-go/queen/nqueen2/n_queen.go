package main

import (
    "fmt"
    "math"
)

// N 皇后
// n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
// 给定一个整数 n，返回 n 皇后不同的解决方案的数量。
// 每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

// 示例
/*
输入: 4
输出: 2
解释: 4 皇后问题存在如下两个不同的解法。
[
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]
*/

type Queen struct {
    positions []int // 记录每一次摆放的位置。下标表示第几个皇后，对应的值表示第几列
    Total int       // 记录解决方案的数量
}

func NewQueen(n int) *Queen {
    positions := make([]int, n)
    return &Queen{positions: positions}
}

// 放置皇后
func (queen *Queen) PutQueen(n int) {
    // 最后一个皇后摆放完成
    if n == len(queen.positions) {
        // 打印出摆放的位置
        // fmt.Printf("%v", queen.positions)

        queen.Total++
        return
    }
    // 有几个位置就有几个皇后
    for i := 0; i < len(queen.positions); i++ {
        // i = 0 时，假设当前皇后可以放在第一列
        // 如果不能放，将进行下一次循环，当前皇后放在下一个位置
        queen.positions[n] = i
        if queen.isNotConflict(n) {
            // 继续摆放下一皇后
            queen.PutQueen(n + 1)
        }
    }
}
// 判断当前皇后是否和已经摆放过的皇后冲突
func (queen *Queen) isNotConflict(n int) bool {
    for i := 0; i < n; i++ {
        // positions[n] == positions[i] 表示在同一列
        // math.Abs(float64(n - i)) == math.Abs(float64(positions[n] - positions[i]) 表示同一斜线
        if queen.positions[n] == queen.positions[i] ||
            math.Abs(float64(n - i)) == math.Abs(float64(queen.positions[n] - queen.positions[i])) {
            return false
        }
    }
    return true
}

func totalNQueens(n int) int {
    queen := NewQueen(n)
    queen.PutQueen(0)
    return queen.Total
}

func main() {
    n := 4
    total := totalNQueens(n)
    fmt.Println(n)
    fmt.Printf("%v\n", total)
}
