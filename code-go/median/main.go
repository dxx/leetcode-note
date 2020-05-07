package median

import (
    "fmt"
    "math"
)

// 寻找两个正序数组的中位数
// 给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。
// 请你找出这两个正序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
// 你可以假设 nums1 和 nums2 不会同时为空。

// 示例1
// nums1 = [1, 3]
// nums2 = [2]
// 则中位数是 2.0

// 示例2
// nums1 = [1, 2]
// nums2 = [3, 4]
// 则中位数是 (2 + 3)/2 = 2.5

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
    // 归并查找，时间复杂度 O(m+n)
    // return mergeSearch(nums1, nums2)

    // 二分查找，时间复杂度 O(log(m+n))
    return binarySearch(nums1, nums2)
}

func binarySearch(nums1 []int, nums2 []int) float64 {
    len1, len2 := len(nums1), len(nums2)
    totalLen := len1 + len2
    if totalLen % 2 != 0 { // 奇数
        median := getKthElement(nums1, nums2, totalLen / 2 + 1)
        return float64(median)
    } else { // 偶数
        median1 := getKthElement(nums1, nums2, totalLen / 2)
        median2 := getKthElement(nums1, nums2, totalLen / 2 + 1)
        return float64(median1 + median2) / 2.0
    }
}
// 获取两个数组中第 k 个元素
func getKthElement(nums1 []int, nums2 []int, k int) int {
    len1, len2 := len(nums1), len(nums2)
    index1, index2 := 0, 0
    for {

        // 数组中的所有元素都被排除，返回另一个数组中第 k 小的元素
        if index1 == len1 {
            return nums2[index2 + k - 1]
        }
        if index2 == len2 {
            return nums1[index1 + k - 1]
        }

        // 如果 k=1，返回两个数组首个元素的最小值
        if k == 1 {
            return int(math.Min(float64(nums1[index1]), float64(nums2[index2])))
        }

        half := k / 2
        newIndex1 := int(math.Min(float64(index1 + half), float64(len1))) - 1
        newIndex2 := int(math.Min(float64(index2 + half), float64(len2))) - 1
        if nums1[newIndex1] <= nums2[newIndex2] {
            // 排除了元素后，k 减少排除元素的个数
            k -= newIndex1 - index1 + 1
            // 记录排除元素后数组的起始下标
            index1 = newIndex1 + 1
        } else {
            k -= newIndex2 - index2 + 1
            index2 = newIndex2 + 1
        }
    }
}

func mergeSearch(nums1 []int, nums2 []int) float64 {
    index1, index2, mergeNumIndex := 0, 0, 0
    len1, len2 := len(nums1), len(nums2)
    value1, value2 := 0, 0
    // 创建一个数组用来保存合并的元素，只需要保存到中间的元素即可
    mergeNums := make([]int, (len1 + len2) / 2 + 1)
    for mergeNumIndex < len(mergeNums) {
        value1 = 1 << 31 - 1
        value2 = 1 << 31 - 1
        if index1 < len1 {
            value1 = nums1[index1]
        }
        if index2 < len2 {
            value2 = nums2[index2]
        }
        if value1 <= value2 {
            mergeNums[mergeNumIndex] = nums1[index1]
            index1++
        } else {
            mergeNums[mergeNumIndex] = nums2[index2]
            index2++
        }
        mergeNumIndex++
    }

    // 奇数取最后一个元素
    if (len1 + len2) % 2 != 0 {
        return float64(mergeNums[len(mergeNums) - 1])
    }
    // 偶数取最后两个元素的平均值
    return (float64(mergeNums[len(mergeNums) - 2]) + float64(mergeNums[len(mergeNums) - 1])) / 2
}

func main() {
    nums1 := []int{1, 3}
    nums2 := []int{2}
    median := findMedianSortedArrays(nums1, nums2)

    fmt.Println(nums1)
    fmt.Println(nums2)
    fmt.Printf("中位数是 %f\n", median)

    nums1 = []int{1, 2}
    nums2 = []int{3, 4}
    median = findMedianSortedArrays(nums1, nums2)

    fmt.Println(nums1)
    fmt.Println(nums2)
    fmt.Printf("中位数是 %f\n", median)
}
