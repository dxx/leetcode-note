package main

import (
    "fmt"
    "sort"
)

// 三数之和
// 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
// 注意: 答案中不可以包含重复的三元组。

// 示例
// 输入: nums = [-1, 0, 1, 2, -1, -4]
// 输出:
// [
//   [-1, 0, 1],
//   [-1, -1, 2]
// ]

func threeSum(nums []int) [][]int {
    sort.Ints(nums)

    results := make([][]int, 0)
    n := len(nums)
    for i := 0; i < n; i++ {
        // 如果元素重复, 跳过
        if i > 0 && nums[i] == nums[i - 1] {
            continue
        }

        target := -nums[i]
        left := i + 1
        right := n - 1
        // 从剩下的数组中寻找两个数之和等于 target 的两个元素
        for left < right {
            leftValue := nums[left]
            rightValue := nums[right]
            sum := leftValue + rightValue
            // 寻找到两个数, 放入结果集中
            if sum == target {
                results = append(results, []int{nums[i], nums[left], nums[right]})

                // 如果元素相同指针后移
                for left < right && nums[left] == leftValue {
                    left++
                }
                // 如果元素相同指针前移
                for left < right && nums[right] == rightValue {
                    right--
                }
                continue
            }
            // 和小于目标值，从右边寻找下一个较大的值
            if sum < target {
                left++
                continue
            }
            // 和大于目标值，从左边寻找下一个较小的值
            right--
        }
    }
    return results
}

func main() {
    nums := []int{-1, 0, 1, 2, -1, -4}
    fmt.Printf("nums = %v\n", nums)

    results := threeSum(nums)
    fmt.Println(results)
}
