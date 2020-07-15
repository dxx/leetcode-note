package main

import (
    "fmt"
    "math"
    "strings"
)

// N 皇后
// n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
// 给定一个整数 n，返回所有不同的 n 皇后问题的解决方案。
// 每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

// 示例
/*
输入: 4
输出: [
[".Q..",  // 解法 1
"...Q",
"Q...",
"..Q."],

["..Q.",  // 解法 2
"Q...",
"...Q",
".Q.."]
]
解释: 4 皇后问题存在两个不同的解法。
*/

type Queen struct {
    positions []int    // 记录每一次摆放的位置。下标表示第几个皇后，对应的值表示第几列
    Outputs [][]string // 保存输出的结果
}

func NewQueen(n int) *Queen {
    positions := make([]int, n)
    return &Queen{positions: positions, Outputs: nil}
}

// 放置皇后
func (queen *Queen) PutQueen(n int) {
    // 最后一个皇后摆放完成
    if n == len(queen.positions) {
        // 打印出摆放的位置
        // fmt.Printf("%v", queen.positions)

        // 根据摆放的位置创建数组
        queen.addSolution(queen.positions)
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

func (queen *Queen) addSolution(positions []int) {
    length := len(positions)
    output := make([]string, length)

    for i := 0; i < length; i++ {
        var stringBuilder strings.Builder
        for j := 0; j < positions[i]; j++ {
            stringBuilder.WriteString(".")
        }
        stringBuilder.WriteString("Q")
        for j := positions[i] + 1; j < length; j++ {
            stringBuilder.WriteString(".")
        }
        output[i] = stringBuilder.String()
    }

    queen.Outputs = append(queen.Outputs, output)
}

func solveNQueens(n int) [][]string {
    queen := NewQueen(n)
    queen.PutQueen(0)
    return queen.Outputs
}

func main() {
    n := 4
    outputs := solveNQueens(n)
    fmt.Println(n)
    fmt.Printf("%v\n", outputs)
}
