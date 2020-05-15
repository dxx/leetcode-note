package threeclosestsuom

import (
    "math"
    "sort"
)

// 最接近的三数之和
// 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。
// 返回这三个数的和。假定每组输入只存在唯一答案。

// 示例
// 输入: nums = [-1,2,1,-4], target = 1
// 输出: 2
// 解释: 与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。

func threeSumClosest(nums []int, target int) int {
    sort.Ints(nums)

    n := len(nums)
    bestSum := math.MaxInt32
    for i := 0; i < n; i++ {
        // 如果元素重复, 跳过
        if i > 0 && nums[i] == nums[i - 1] {
            continue
        }

        left := i + 1
        right := n - 1
        // 从剩下的数组中寻找所有两个数, 加上第一个数之和最接近 target 的目标和
        for left < right {
            leftValue := nums[left]
            rightValue := nums[right]
            sum := nums[i] + leftValue + rightValue

            // 相等直接返回
            if sum == target {
                return target
            }
            // 比较当前三数之和上一次和的差值
            if abs(sum - target) < abs(bestSum - target) {
                bestSum = sum
            }

            // 和小于目标值，从右边寻找下一个较大的值
            if sum < target {
                left++
                // 如果元素相同指针后移
                for left < right && nums[left] == leftValue {
                    left++
                }
                continue
            }
            // 和大于目标值，从左边寻找下一个较小的值
            right--
            // 如果元素相同指针前移
            for left < right && nums[right] == rightValue {
                right--
            }
        }
    }
    return bestSum
}

func abs(a int) int {
    if a > 0 {
        return a
    }
    return -a
}
