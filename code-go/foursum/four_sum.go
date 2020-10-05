package main

import (
    "fmt"
    "math"
    "sort"
)

// 四数之和
// 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d 
// 使得 a + b + c + d 的值与 target 相等？找出所有满足条件且不重复的四元组。
// 注意: 答案中不可以包含重复的四元组。

// 示例
// 输入: nums = [1, 0, -1, 0, -2, 2]，和 target = 0
// 输出:
// [
//   [-1,  0, 0, 1],
//   [-2, -1, 1, 2],
//   [-2,  0, 0, 2]
// ]

var res [][]int

func fourSum(nums []int, target int) [][]int {
    res = make([][]int, 0)

    sort.Ints(nums)
    // 记录当前第 n 个数
    records := make([]int, 4)
    for i := 0; i < len(records); i++ {
        records[i] = math.MinInt32
    }
    nSum(nums, 0, len(nums) - 1, target, 4, records)
    return res
}

// 计算 n 个数之和
func nSum(nums []int, start, end, target, n int, records []int) {
    recordLen := len(records)
    if n <= 2 {
        // 计算两数之和
        results := twoSum(nums, start, end, target)
        // 存在两数之和等于目标值
        if len(results) > 0 {
            // 循环遍历结果, 将结果添加到最后的结果中
            for i := 0; i < len(results);i++ {
                records[recordLen - 2] = results[i][0]
                records[recordLen - 1] = results[i][1]
                res = append(res, append([]int{}, records...))
            }
        }
        return
    }
    i := start
    for ; i <= end; i++ {
        // 如果元素重复, 跳过
        if nums[i]  == records[recordLen - n] {
            continue
        }

        // 修改当前第 n 个数
        records[recordLen - n] = nums[i]
        if i + 1 < end {
            // 继续计算第 n - 1 个数之和
            nSum(nums, i + 1, end, target - nums[i], n - 1, records)
        }

        // 表示下一次重新计算 n 数之和, 将数组元素重置
        if n == recordLen {
            for j := 1; j < recordLen; j++ {
                records[j] = math.MinInt32
            }
        }
    }
}

// 计算两数之和
func twoSum(nums []int, start, end, target int) [][]int {
    var results [][]int
    left, right := start, end
    sum := 0
    for left < right {
        leftVal := nums[left]
        rightVal := nums[right]
        sum = leftVal + rightVal
        // 如果相等，将元素放入切片
        if sum == target {
            results = append(results, []int{leftVal, rightVal})

            // 如果元素相同指针后移
            for left < right && nums[left] == leftVal {
                left++
            }
            // 如果元素相同指针前移
            for left < right && nums[right] == rightVal {
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
    return results
}

func main() {
    nums := []int{1, 0, -1, 0, -2, 2}
    target := 0
    fmt.Printf("nums = %v, target = %v\n", nums, target)

    sum := fourSum(nums, target)
    fmt.Println(sum)
}
