package com.mcx;

/**
 * Created by mcx on 2020-07-15.
 *
 * N 皇后
 * n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 * 给定一个整数 n，返回 n 皇后不同的解决方案的数量。
 * 每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
 *
 * 示例
 * 输入: 4
 * 输出: 2
 * 解释: 4 皇后问题存在如下两个不同的解法。
 * [
 *  [".Q..",  // 解法 1
 *   "...Q",
 *   "Q...",
 *   "..Q."],
 *
 *  ["..Q.",  // 解法 2
 *   "Q...",
 *   "...Q",
 *   ".Q.."]
 * ]
 */
public class NQueen2 {

    private int[] positions; // 记录每一次摆放的位置。下标表示第几个皇后，对应的值表示第几列
    private int total; // 记录解决方案的数量

    public void putQueen(int n) {
        // 最后一个皇后摆放完成
        if (n == this.positions.length) {
            // 打印出摆放的位置
            // System.out.println(Arrays.toString(this.positions));

            this.total++;
            return;
        }

        // 有几个位置就有几个皇后
        for (int i = 0; i < this.positions.length; i++) {
            // i = 0 时，假设当前皇后可以放在第一列
            // 如果不能放，将进行下一次循环，当前皇后放在下一个位置
            this.positions[n] = i;
            if (this.isNotConflict(n)) {
                // 继续摆放下一皇后
                this.putQueen(n + 1);
            }
        }
    }

    /**
     * 判断当前皇后是否和已经摆放过的皇后冲突
     */
    private boolean isNotConflict(int n) {
        for (int i = 0; i < n; i++) {
            // this.positions[n] == this.positions[i] 表示在同一列
            // Math.abs(n - i) == Math.abs(this.positions[n] - this.positions[i]) 表示同一斜线
            if (this.positions[n] == this.positions[i] ||
            Math.abs(n - i) == Math.abs(this.positions[n] - this.positions[i])) {
                return false;
            }
        }
        return true;
    }

    public int solveNQueens(int n) {
        this.positions = new int[n];
        this.putQueen(0);
        return this.total;
    }

    public static void main(String[] args) {
        int n = 4;
        NQueen2 queen = new NQueen2();
        int total = queen.solveNQueens(n);
        System.out.println(n);
        System.out.println(total);
    }
}
