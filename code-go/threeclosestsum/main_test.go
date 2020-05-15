package threeclosestsuom

import "testing"

func TestThreeSumClosest(t *testing.T) {
    nums := []int{-1, 2, 1, -4}
    target := 1
    t.Logf("nums = %v, target = %v\n", nums, target)

    sum := threeSumClosest(nums, target)
    t.Log(sum)
}
