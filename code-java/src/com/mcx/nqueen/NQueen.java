package com.mcx.nqueen;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * N 皇后
 * n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 * 给定一个整数 n，返回所有不同的 n 皇后问题的解决方案。
 * 每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
 *
 * 示例
 * 输入: 4
 * 输出: [
 * [".Q..",  // 解法 1
 * "...Q",
 * "Q...",
 * "..Q."],
 *
 * ["..Q.",  // 解法 2
 * "Q...",
 * "...Q",
 * ".Q.."]
 * ]
 * 解释: 4 皇后问题存在两个不同的解法。
 */
public class NQueen {

    private int[] positions; // 记录每一次摆放的位置。下标表示第几个皇后，对应的值表示第几列
    private List<List<String>> outputs; // 保存输出的结果

    public void putQueen(int n) {
        // 最后一个皇后摆放完成
        if (n == this.positions.length) {
            // 打印出摆放的位置
            // System.out.println(Arrays.toString(this.positions));

            // 根据摆放的位置创建数组
            this.addSolution(this.positions);
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

    private void addSolution(int[] positions) {
        int length = positions.length;
        List<String> output = new ArrayList<>();

        for (int position : positions) {
            StringBuilder sb = new StringBuilder();
            for (int j = 0; j < position; j++) {
                sb.append(".");
            }
            sb.append("Q");
            for (int j = position + 1; j < length; j++) {
                sb.append(".");
            }
            output.add(sb.toString());
        }
        this.outputs.add(output);
    }

    public List<List<String>> solveNQueens(int n) {
        this.positions = new int[n];
        this.outputs = new ArrayList<>();
        this.putQueen(0);
        return this.outputs;
    }

    public static void main(String[] args) {
        int n = 4;
        NQueen queen = new NQueen();
        List<List<String>> outputs = queen.solveNQueens(n);
        System.out.println(n);

        outputs.forEach(output -> System.out.println(Arrays.toString(output.toArray())));
    }
}
