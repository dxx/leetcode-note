package com.dxx.uniquepaths;

import java.util.Arrays;

/**
 * 不同路径
 * 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
 * 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
 * 问总共有多少条不同的路径？
 *
 * 示例
 * 输入: m = 3, n = 2
 * 输出: 3
 * 解释:
 * 从左上角开始，总共有 3 条路径可以到达右下角。
 * 1. 向右 -> 向右 -> 向下
 * 2. 向右 -> 向下 -> 向右
 * 3. 向下 -> 向右 -> 向右
 *
 * 输入: m = 7, n = 3
 * 输出: 28
 */
public class Main {

    public static int uniquePaths(int m, int n) {
        int row = n, col  = m;

        // 使用一维数组代替二维数组，节省空间
        int[] paths = new int[col];
        // 初始化为 1
        Arrays.fill(paths, 1);

        // 起始点 0,0
        int startX = 0, startY = 0;

        for (int i = startY + 1; i < row; i++) {
            for (int j = startX + 1; j < col; j++) {
                // paths 滑动数组
                paths[j] += paths[j - 1];
            }
        }
        // 返回最后的路径数目
        return paths[col - 1];
    }

    public static void main(String[] args) {
        int m = 3;
        int n = 2;
        System.out.printf("m = %d, n = %d\n", m, n);
        int count = uniquePaths(m, n);
        System.out.println(count);

        m = 7;
        n = 3;
        System.out.printf("m = %d, n = %d\n", m, n);
        count = uniquePaths(m, n);
        System.out.println(count);
    }
}
