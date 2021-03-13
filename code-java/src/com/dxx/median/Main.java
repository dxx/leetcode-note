package com.dxx.median;

import java.util.Arrays;

/**
 * 寻找两个正序数组的中位数
 * 给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。
 * 请你找出这两个正序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
 * 你可以假设 nums1 和 nums2 不会同时为空。
 *
 * 示例1
 * nums1 = [1, 3]
 * nums2 = [2]
 * 则中位数是 2.0
 *
 * 示例2
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 * 则中位数是 (2 + 3)/2 = 2.5
 */
public class Main {

    public static double findMedianSortedArrays(int[] nums1, int[] nums2) {
        // 归并查找，时间复杂度 O(m+n)
        // return mergeSearch(nums1, nums2);

        // 二分查找，时间复杂度 O(log(m+n))
        return binarySearch(nums1, nums2);
    }

    public static double binarySearch(int[] nums1, int[] nums2) {
        int len1 = nums1.length, len2 = nums2.length;
        int totalLen = len1 + len2;
        if (totalLen % 2 != 0) { // 奇数
            return getKthElement(nums1, nums2, totalLen / 2 + 1);
        } else { // 偶数
            double median1 = getKthElement(nums1, nums2, totalLen / 2);
            double median2 = getKthElement(nums1, nums2, totalLen / 2 + 1);
            return (median1 + median2) / 2.0;
        }
    }

    /**
     * 获取两个数组中第 k 个元素
     */
    private static int getKthElement(int[] nums1, int[] nums2, int k) {
        int len1 = nums1.length, len2 = nums1.length;
        int index1 = 0, index2 = 0;
        while (true) {

            // 数组中的所有元素都被排除，返回另一个数组中第 k 小的元素
            if (index1 == len1) {
                return nums2[index2 + k - 1];
            }
            if (index2 == len2) {
                return nums1[index1 + k - 1];
            }

            // 如果 k=1，返回两个数组首个元素的最小值
            if (k == 1) {
                return Math.min(nums1[index1], nums2[index2]);
            }

            int half = k / 2;
            int newIndex1 = Math.min(index1 + half, len1) - 1;
            int newIndex2 = Math.min(index2 + half, len2) - 1;
            if (nums1[newIndex1] <= nums2[newIndex2]) {
                // 排除了元素后，k 减少排除元素的个数
                k -= newIndex1 - index1 + 1;
                // 记录排除元素后数组的起始下标
                index1 = newIndex1 + 1;
            } else {
                k -= newIndex2 - index2 + 1;
                index2 = newIndex2 + 1;
            }
        }
    }

    public static double mergeSearch(int[] nums1, int[] nums2) {
        int index1 = 0, index2 = 0, mergeNumIndex = 0;
        int len1 = nums1.length, len2 = nums2.length;
        int value1 = 0, value2 = 0;
        // 创建一个数组用来保存合并的元素，只需要保存到中间的元素即可
        int[] mergeNums = new int[(len1 + len2) / 2 + 1];
        while (mergeNumIndex < mergeNums.length) {
            value1 = Integer.MAX_VALUE;
            value2 = Integer.MAX_VALUE;
            if (index1 < len1) {
                value1 = nums1[index1];
            }
            if (index2 < len2) {
                value2 = nums1[index2];
            }
            if (value1 <= value2) {
                mergeNums[mergeNumIndex++] = nums1[index1++];
            } else {
                mergeNums[mergeNumIndex++] = nums2[index2++];
            }
        }

        // 奇数取最后一个元素
        if ((len1 + len2) % 2 != 0) {
            return mergeNums[mergeNums.length - 1];
        }
        // 偶数取最后两个元素的平均值
        return (mergeNums[mergeNums.length - 2] + mergeNums[mergeNums.length - 1]) / 2.0;
    }

    public static void main(String[] args) {
        int[] nums1 = new int[]{1, 3};
        int[] nums2 = new int[]{2};
        double median = findMedianSortedArrays(nums1, nums2);

        System.out.println(Arrays.toString(nums1));
        System.out.println(Arrays.toString(nums2));
        System.out.printf("中位数是 %f\n", median);

        nums1 = new int[]{1, 2};
        nums2 = new int[]{3, 4};
        median = findMedianSortedArrays(nums1, nums2);

        System.out.println(Arrays.toString(nums1));
        System.out.println(Arrays.toString(nums2));
        System.out.printf("中位数是 %f\n", median);
    }
}
