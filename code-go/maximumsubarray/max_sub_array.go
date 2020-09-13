package main

import "fmt"

// 最大子序和
// 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

// 示例
// 输入: [-2,1,-3,4,-1,2,1,-5,4]
// 输出: 6
// 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。

func maxSubArray(nums []int) int {
    // 记录以每个元素结尾的最大值数组
    maxArray := make([]int, len(nums))
    // 第一个元结尾的最大值
    maxArray[0] = nums[0]
    // 初始化最大值
    maxValue := nums[0]
    for i := 1; i < len(nums); i++ {
        maxArray[i] = nums[i]
        // 前一个元素结尾的最大值
        if maxArray[i - 1] + nums[i] > nums[i] {
            maxArray[i] += maxArray[i - 1]
        }
        // 判断最大值
        if maxArray[i] > maxValue {
            maxValue = maxArray[i]
        }
    }
    return maxValue
}

func main() {
    nums := []int{-2, 1, -3, 4, -1, 2, 1, -5, 4}
    fmt.Println(nums)

    value := maxSubArray(nums)
    fmt.Println(value)
}
