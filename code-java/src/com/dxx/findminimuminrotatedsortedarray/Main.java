package com.dxx.findminimuminrotatedsortedarray;

import java.util.Arrays;

/**
 * 寻找旋转排序数组中的最小值
 * 假设按照升序排序的数组在预先未知的某个点上进行了旋转。 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2],
 * 请找出其中最小的元素。
 *
 * 你可以假设数组中不存在重复元素。
 *
 * 示例
 * 输入: [3,4,5,1,2]
 * 输出: 1
 *
 * 输入: [4,5,6,7,0,1,2]
 * 输出: 0
 */
public class Main {
    public static int findMin(int[] nums) {
        int start = 0, end = nums.length - 1;
        // 二分查找
        while (start < end) {
            int mid = (start + end) >> 1;
            // 最小值比中间元素大，说明最小值在中间元素的左边
            if (nums[end] > nums[mid]) {
                end = mid;
            } else {
                // 最小值比中间元素小，最小值在中间元素的右边
                start = mid + 1;
            }
        }
        return nums[end];
    }

    public static void main(String[] args) {
        int[] nums = new int[]{3, 4, 5, 1, 2};
        int min = findMin(nums);

        System.out.println(Arrays.toString(nums));
        System.out.println(min);

        nums = new int[]{4, 5, 6, 7, 0, 1, 2};
        min = findMin(nums);

        System.out.println(Arrays.toString(nums));
        System.out.println(min);
    }
}
