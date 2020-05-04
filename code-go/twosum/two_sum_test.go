package twosum

import "testing"

func TestTwoSum(t *testing.T) {
    nums := []int{2, 7, 11, 15}
    target := 9
    result := twoSum(nums, target)
    t.Log(result)
}
