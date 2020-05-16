package foursum

import (
    "testing"
)

func TestFourSum(t *testing.T) {
    nums := []int{1, 0, -1, 0, -2, 2}
    target := 0
    t.Logf("nums = %v, target = %v\n", nums, target)

    sum := fourSum(nums, target)
    t.Log(sum)
}
