package com.mcx;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

/**
 * Created by mcx on 2020-06-28.
 *
 * 两数之和
 * 给定一个整数数组 `nums` 和一个目标值 `target`，请你在该数组中找出和为目标值的那**两个**整数，并返回他们的数组下标。
 * 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
 *
 * 示例
 * 给定 nums = [2, 7, 11, 15], target = 9
 * 因为 nums[0] + nums[1] = 2 + 7 = 9
 * 所以返回 [0, 1]
 */
public class TwoSum {

    public static int[] towSum(int[] nums, int target) {
        int[] indexes = new int[2];
        Map<Integer, Integer> numMap = new HashMap<>();
        // 时间复杂度 O(n), 空间复杂度 O(n)
        for (int i = 0; i < nums.length; i++) {
            // 计算补数
            int n = target - nums[i];
            Integer numIndex = numMap.get(n);
            // 判断补数是否存在
            if (numIndex != null) {
                // 补数存在，将补数的小标和当前数的下标放入数组中
                indexes[0] = numIndex;
                indexes[1] = i;
                break;
            }
            // 将当前元素放入 map 中，方便后续判断
            numMap.put(nums[i], i);
        }
        return indexes;
    }

    public static void main(String[] args) {
        int[] nums = {2, 7, 11, 15};
        int target = 9;
        int[] result = towSum(nums, target);
        System.out.println(Arrays.toString(result));
    }
}
