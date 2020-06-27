package findminimuminrotatedsortedarray

import "testing"

func TestFindMin2(t *testing.T) {
    nums := []int{1, 3, 5}
    min := findMin2(nums)

    t.Log(nums)
    t.Log(min)

    nums = []int{2, 2, 2, 0, 1}
    min = findMin2(nums)

    t.Log(nums)
    t.Log(min)
}
