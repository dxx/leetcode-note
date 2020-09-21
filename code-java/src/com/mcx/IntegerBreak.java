package com.mcx;

/**
 * Created by mcx on 2020-09-20.
 *
 * 整数拆分
 * 给定一个正整数 n，将其拆分为至少两个正整数的和，并使这些整数的乘积最大化。返回你可以获得的最大乘积。
 *
 * 示例
 * 输入: 2
 * 输出: 1
 * 解释: 2 = 1 + 1, 1 × 1 = 1。
 *
 * 输入: 10
 * 输出: 36
 * 解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。
 */
public class IntegerBreak {

    public static int integerBreak(int n) {
        int[] dp = new int[n + 1];
        for (int i = 2; i <= n; i++) {
            for (int j = 1; j < i; j++) {
                // 求拆分成 j 和 i - j 或者 j 和 i - j (能继续拆分)
                // 和上一次拆分 dp[i] (也就是拆分成 1, j - 1 ) 中的最大值
                dp[i] = Math.max(dp[i], Math.max(j * (i - j), j * dp[i - j]));
            }
        }
        return dp[n];
    }

    public static void main(String[] args) {
        int n = 2;
        System.out.println(n);

        int max = integerBreak(n);
        System.out.println(max);

        n = 10;
        System.out.println(n);

        max = integerBreak(n);
        System.out.println(max);
    }
}
