package median

import "testing"

func TestFindMedianSortedArrays(t *testing.T) {
    nums1 := []int{1, 3}
    nums2 := []int{2}
    median := findMedianSortedArrays(nums1, nums2)

    t.Log(nums1)
    t.Log(nums2)
    t.Logf("中位数是 %f\n", median)

    nums1 = []int{1, 2}
    nums2 = []int{3, 4}
    median = findMedianSortedArrays(nums1, nums2)

    t.Log(nums1)
    t.Log(nums2)
    t.Logf("中位数是 %f\n", median)
}
