package insertposition

// 搜索插入位置
// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
//
// 你可以假设数组中无重复元素。

// 示例
// 输入: [1,3,5,6], 5
// 输出: 2

// 输入: [1,3,5,6], 2
// 输出: 1

// 输入: [1,3,5,6], 7
// 输出: 4

// 输入: [1,3,5,6], 0
// 输出: 0

func searchInsert(nums []int, target int) int {
    start, end := 0, len(nums) - 1
    // 下标设置为数组长度，如果 target 大于数组中的所有元素，此时插入位置就是数组的长度
    index := len(nums)
    for start <= end {
        mid := (start + end) >> 1
        if target < nums[mid] {
            // 修改插入的下标
            index = mid
            end = mid - 1
        } else if target > nums[mid] {
            start = mid + 1
        } else {
            index = mid
            break
        }
    }
    return index
}
