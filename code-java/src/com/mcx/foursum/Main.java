package com.mcx.foursum;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * 四数之和
 * 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d 
 * 使得 a + b + c + d 的值与 target 相等？找出所有满足条件且不重复的四元组。
 * 注意: 答案中不可以包含重复的四元组。
 *
 * 示例
 * 输入: nums = [1, 0, -1, 0, -2, 2]，和 target = 0
 * 输出:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 */
public class Main {
    
    public static List<List<Integer>> fourSum(int[] nums, int target) {
        List<List<Integer>> res = new ArrayList<>();

        Arrays.sort(nums);
        int[] records = new int[4];
        Arrays.fill(records, Integer.MIN_VALUE);
        nSum(nums, 0, nums.length - 1, target, 4, records, res);
        return res;
    }

    /**
     * 计算 n 个数之和
     */
    public static void nSum(int[] nums, int start, int end, int target, int n, int[] records, List<List<Integer>> res) {
        int recordLen = records.length;
        if (n <= 2) {
            // 计算两数之和
            List<List<Integer>> results = twoSum(nums, start, end, target);
            // 存在两数之和等于目标值
            if (results.size() > 0) {
                // 循环遍历结果, 将结果添加到最后的结果中
                for (List<Integer> result : results) {
                    records[recordLen - 2] = result.get(0);
                    records[recordLen - 1] = result.get(1);

                    List<Integer> list = new ArrayList<>();
                    for (int record : records) {
                        list.add(record);
                    }
                    res.add(list);
                }
            }
            return;
        }
        int i = start;
        for (; i < end; i++) {
            // 如果元素重复, 跳过
            if (nums[i]  == records[recordLen - n]) {
                continue;
            }

            // 修改当前第 n 个数
            records[recordLen - n] = nums[i];
            if (i + 1 < end) {
                // 继续计算第 n - 1 个数之和
                nSum(nums, i + 1, end, target - nums[i], n - 1, records, res);
            }

            // 表示下一次重新计算 n 数之和, 将数组元素重置
            if (n == recordLen) {
                for (int j = 1; j < recordLen; j++) {
                    records[j] = Integer.MIN_VALUE;
                }
            }
        }
    }

    /**
     * 计算两数之和
     */
    public static List<List<Integer>> twoSum(int[] nums, int start, int end, int target) {
        List<List<Integer>> results = new ArrayList<>();
        int left = start, right = end;
        int sum;
        while (left < right) {
            int leftVal = nums[left];
            int rightVal = nums[right];
            sum = leftVal + rightVal;
            // 如果相等，将元素放入集合
            if (sum == target) {
                List<Integer> list = new ArrayList<>();
                list.add(leftVal);
                list.add(rightVal);
                results.add(list);

                // 如果元素相同指针后移
                while (left < right && nums[left] == leftVal) {
                    left++;
                }
                // 如果元素相同指针前移
                while (left < right && nums[right] == rightVal) {
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
        return results;
    }

    public static void main(String[] args) {
        int[] nums = new int[]{1, 0, -1, 0, -2, 2};
        int target = 0;
        System.out.printf("nums = %s, target = %d\n", Arrays.toString(nums), target);

        List<List<Integer>> results = fourSum(nums, target);
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
