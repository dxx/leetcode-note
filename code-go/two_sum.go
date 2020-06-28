package main

import "fmt"

// 两数之和
// 给定一个整数数组 `nums` 和一个目标值 `target`，请你在该数组中找出和为目标值的那**两个**整数，并返回他们的数组下标。
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

// 示例
// 给定 nums = [2, 7, 11, 15], target = 9
// 因为 nums[0] + nums[1] = 2 + 7 = 9
// 所以返回 [0, 1]

func twoSum(nums []int, target int) []int {
    var indexes []int

    // 时间复杂度 O((n^2-n)/2)
    /*for i := 0; i < len(nums) - 1; i++ {
        for j := i + 1; j < len(nums); j++ {
            if nums[i] + nums[j] == target {
                indexes = append(indexes, i, j)
                break
            }
        }
    }*/

    // 时间复杂度 O(n)
    numMap := make(map[int]int)
    for i, v := range nums {
        // 计算补数
        n := target - v
        numIndex, ok := numMap[n]
        // 判断补数是否存在
        if ok {
            // 补数存在，将补数的小标和当前数的下标放入数组中
            indexes = append(indexes, numIndex, i)
            break
        }
        // 将当前元素放入 map 中，方便后续判断
        numMap[v] = i
    }
    return indexes
}

func main()  {
    nums := []int{2, 7, 11, 15}
    target := 9
    result := twoSum(nums, target)

    fmt.Println(result)
}
