package com.mcx;

import java.util.Arrays;

/**
 * Created by mcx on 2020-10-08.
 *
 * 颜色分类
 *
 * 给定一个包含红色、白色和蓝色，一共 n 个元素的数组，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
 * 此题中，我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
 *
 * 注意: 不能使用代码库中的排序函数来解决这道题。
 *
 * 进阶:
 * 一个直观的解决方案是使用计数排序的两趟扫描算法。
 * 首先，迭代计算出0、1 和 2 元素的个数，然后按照0、1、2的排序，重写当前数组。
 * 你能想出一个仅使用常数空间的一趟扫描算法吗？
 *
 * 示例
 * 输入: [2, 0, 2, 1, 1, 0]
 * 输出: [0, 0, 1, 1, 2, 2]
 */
public class SortColors {

    public static void sortColors(int[] nums) {
        // p0 表示 0 要放置的下标, p1 表示 1 要放置的下标
        int p0 = 0, p1 = 0;
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] == 0) {
                // 交换 i 和 p0 指向的元素
                swap(nums, i, p0);
                // 有可能 p0 指向的元素为 1, 跟 i 发生了交换
                if (p0 < p1) {
                    // 将 p1 和 i 交换
                    swap(nums, i, p1);
                }
                p0++;
                p1++;
            } else if (nums[i] == 1) {
                // 交换 i 和 p1 指向的元素
                swap(nums, i, p1);
                p1++;
            }
        }
    }

    public static void swap(int[] nums, int a, int b) {
        int temp = nums[a];
        nums[a] = nums[b];
        nums[b] = temp;
    }

    public static void main(String[] args) {
        int[] nums = new int[]{2, 0, 2, 1, 1, 0};
        System.out.println(Arrays.toString(nums));

        sortColors(nums);
        System.out.println(Arrays.toString(nums));
    }
}