package main

import "fmt"

// 寻找旋转排序数组中的最小值 II
// 假设按照升序排序的数组在预先未知的某个点上进行了旋转。例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2],
// 请找出其中最小的元素。

// 注意数组中可能存在重复的元素。

// 示例
// 输入: [1,3,5]
// 输出: 1

// 输入: [2,2,2,0,1]
// 输出: 0

func findMin2(nums []int) int {
    start, end := 0, len(nums) - 1
    // 二分查找
    for start < end {
        mid := (start + end) >> 1
        // 最小值比中间元素大，说明最小值在中间元素的左边
        if nums[end] > nums[mid] {
            end = mid
        } else if nums[end] < nums[mid] {
            // 最小值比中间元素小，最小值在中间元素的右边
            start = mid + 1
        } else {
            // 相等，end 前移一位
            end--
        }
    }
    return nums[end]
}

func main() {
    nums := []int{1, 3, 5}
    min := findMin2(nums)

    fmt.Println(nums)
    fmt.Println(min)

    nums = []int{2, 2, 2, 0, 1}
    min = findMin2(nums)

    fmt.Println(nums)
    fmt.Println(min)
}
