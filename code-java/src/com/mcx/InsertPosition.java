package com.mcx;

import java.util.Arrays;

/**
 * Created by mcx on 2020-07-17.
 *
 * 搜索插入位置
 * 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
 *
 * 你可以假设数组中无重复元素。
 *
 * 示例
 * 输入: [1,3,5,6], 5
 * 输出: 2
 *
 * 输入: [1,3,5,6], 2
 * 输出: 1
 *
 * 输入: [1,3,5,6], 7
 * 输出: 4
 *
 * 输入: [1,3,5,6], 0
 * 输出: 0
 */
public class InsertPosition {

    public static int searchInsert(int[] nums, int target) {
        int start = 0, end = nums.length - 1;
        // 下标设置为数组长度，如果 target 大于数组中的所有元素，此时插入位置就是数组的长度
        int index = nums.length;
        while (start <= end) {
            int mid = (start + end) >> 1;
            if (target < nums[mid]) {
                // 修改插入的下标
                index = mid;
                end = mid - 1;
            } else if (target > nums[mid]) {
                start = mid + 1;
            } else {
                index = mid;
                break;
            }
        }
        return index;
    }

    public static void main(String[] args) {
        int[] nums = new int[]{1, 3, 5, 6};

        int target = 5;
        int index = searchInsert(nums, target);

        System.out.printf("%s, %d\n", Arrays.toString(nums), target);
        System.out.println(index);

        target = 2;
        index = searchInsert(nums, target);

        System.out.printf("%s, %d\n", Arrays.toString(nums), target);
        System.out.println(index);

        target = 7;
        index = searchInsert(nums, target);

        System.out.printf("%s, %d\n", Arrays.toString(nums), target);
        System.out.println(index);

        target = 0;
        index = searchInsert(nums, target);

        System.out.printf("%s, %d\n", Arrays.toString(nums), target);
        System.out.println(index);
    }
}
