package uniquepaths

// 不同路径
// 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
// 问总共有多少条不同的路径？

// 示例
// 输入: m = 3, n = 2
// 输出: 3
// 解释:
// 从左上角开始，总共有 3 条路径可以到达右下角。
// 1. 向右 -> 向右 -> 向下
// 2. 向右 -> 向下 -> 向右
// 3. 向下 -> 向右 -> 向右

// 输入: m = 7, n = 3
// 输出: 28

func uniquePaths(m int, n int) int {
    //row, col := n, m
    //// 用来保存原数组从起点开始到对应点的路径数目
    //paths := make([][]int, row)
    //for i := 0; i < len(paths); i++ {
    //  paths[i] = make([]int, col)
    //}
    //// 起始点 0,0
    //startX, startY := 0, 0
    //
    //// 第一列所有点的路径数目
    //for i := startY; i < row; i++ {
    //  paths[i][0] = 1
    //}
    //
    //// 第一行所有点的路径数目
    //for j := startX; j < col; j++ {
    //  paths[0][j] = 1
    //}
    //
    //// 其它点的路径数目等于上边和左边点的路径数目之和
    //for i := startY + 1; i < row; i++ {
    //  for j := startX + 1; j < col; j++ {
    //      paths[i][j] = paths[i - 1][j] + paths[i][j - 1]
    //  }
    //}
    //// 返回最后一个点的路径数目
    //return paths[row - 1][col - 1]

    row, col := n, m

    // 使用一维数组代替二维数组，节省空间
    paths := make([]int, col)
    // 初始化为 1
    for i := 0; i < len(paths); i++ {
       paths[i] = 1
    }

    // 起始点 0,0
    startX, startY := 0, 0

    for i := startY + 1; i < row; i++ {
       for j := startX + 1; j < col; j++ {
           // paths 滑动数组
           paths[j] += paths[j - 1]
       }
    }
    // 返回最后的路径数目
    return paths[col - 1]
}
