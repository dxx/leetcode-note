package com.dxx.threesum;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * 三数之和
 * 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
 * 注意: 答案中不可以包含重复的三元组。
 *
 * 示例
 * 输入: nums = [-1, 0, 1, 2, -1, -4]
 * 输出:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 */
public class Main {

    public static List<List<Integer>> threeSum(int[] nums) {
        Arrays.sort(nums);

        List<List<Integer>> results = new ArrayList<>();
        int n = nums.length;
        for (int i = 0; i < n; i++) {
            // 如果元素重复, 跳过
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }

            int target = -nums[i];
            int left = i + 1;
            int right = n - 1;
            // 从剩下的数组中寻找所有两个数之和等于 target 的两个元素
            while (left < right) {
                int leftValue = nums[left];
                int rightValue = nums[right];
                int sum = leftValue + rightValue;
                // 寻找到两个数, 放入结果集中
                if (sum == target) {
                    List<Integer> list = new ArrayList<>();
                    list.add(nums[i]);
                    list.add(nums[left]);
                    list.add(nums[right]);
                    results.add(list);

                    // 如果元素相同指针后移
                    while (left < right && nums[left] == leftValue) {
                        left++;
                    }
                    // 如果元素相同指针前移
                    while (left < right && nums[right] == rightValue) {
                        right--;
                    }
                    continue;
                }
                // 和小于目标值，从右边寻找下一个较大的值
                if (sum < target) {
                    left++;
                    continue;
                }
                // 和大于目标值，从左边寻找下一个较小的值
                right--;
            }
        }
        return results;
    }

    public static void main(String[] args) {
        int[] nums = new int[]{-1, 0, 1, 2, -1, -4};
        System.out.printf("nums = %s\n", Arrays.toString(nums));

        List<List<Integer>> results = threeSum(nums);
        StringBuilder sb = new StringBuilder("[");
        for (List<Integer> ints : results) {
            sb.append("[");
            for (Integer in : ints) {
                sb.append(in).append(", ");
            }
            sb.deleteCharAt(sb.length() - 1);
            sb.deleteCharAt(sb.length() - 1);
            sb.append("]");
            sb.append(", ");
        }
        sb.deleteCharAt(sb.length() - 1);
        sb.deleteCharAt(sb.length() - 1);
        sb.append("]");
        System.out.println(sb.toString());
    }
}
