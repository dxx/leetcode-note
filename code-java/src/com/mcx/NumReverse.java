package com.mcx;

/**
 * Created by mcx on 2020-06-29.
 *
 * 整数反转
 * 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
 * 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31, 2^31 − 1]。
 * 请根据这个假设，如果反转后整数溢出那么就返回 0。
 *
 * 示例
 * 输入: 123
 * 输出: 321
 *
 * 输入: -123
 * 输出: -321
 *
 * 输入: 120
 * 输出: 21
 */
public class NumReverse {

    public static int reverse(int num) {
        int reverseNum = 0;
        int prevNum = 0;
        int digit;
        while (num != 0) {
            // 计算出每一位的数字
            digit = num % 10;
            // 每次计算出一位数字后除以 10
            num /= 10;
            // 将反转后的数字增加 10 倍，把反转后的数字上累加到后面
            reverseNum = reverseNum * 10 + digit;

            // 当前的数除以 10 后和上一次循环的数不相等说明越界了
            if (reverseNum / 10 != prevNum) {
                return 0;
            }
            prevNum = reverseNum;
        }

        return reverseNum;
    }

    public static void main(String[] args) {
        int num = 123;
        int result = reverse(num);
        System.out.println(result);

        num = -123;
        result = reverse(num);
        System.out.println(result);

        num = 120;
        result = reverse(num);
        System.out.println(result);

        num = 2147483647;
        result = reverse(num);
        System.out.println(result);
    }
}