package com.dxx.twosum;

import java.util.Arrays;

/**
 * 两数之和 II - 输入有序数组
 * 给定一个已按照 **升序排列** 的有序数组，找到两个数使得它们相加之和等于目标数。
 * 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
 *
 * 说明:
 * 返回的下标值（index1 和 index2）不是从零开始的。
 * 你可以假设每个输入只对应唯一的答案，而且你不可以重复使用相同的元素。
 *
 * 示例
 * 输入: numbers = [2, 7, 11, 15], target = 9
 * 输出: [1,2]
 * 解释: 2 与 7 之和等于目标数 9 。因此 index1 = 1, index2 = 2 。
 */
public class Main2 {
    public static int[] towSum(int[] numbers, int target) {
        int[] indexes = new int[2];
        int left = 0, right = numbers.length - 1;
        int sum = 0;
        // 时间复杂度 O(n), 空间复杂度 O(1)
        while (left < right) {
            sum = numbers[left] + numbers[right];
            // 如果相等，将索引放入数组，跳出循环
            if (sum == target) {
                indexes[0] = left + 1;
                indexes[1] = right + 1;
                break;
            }
            // 和小于目标值，从右边寻找下一个较大的值
            if (sum < target) {
                left++;
                continue;
            }
            // 和大于目标值，从左边寻找下一个较小的值
            right--;
        }
        return indexes;
    }

    public static void main(String[] args) {
        int[] numbers = new int[]{2, 7, 11, 15};
        int target = 9;

        int[] result = towSum(numbers, target);

        System.out.printf("numbers = %s, target = %d\n", Arrays.toString(numbers), target);
        System.out.println(Arrays.toString(result));
    }
}
