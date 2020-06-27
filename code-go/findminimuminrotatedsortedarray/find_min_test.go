package findminimuminrotatedsortedarray

import "testing"

func TestFindMin(t *testing.T) {
    nums := []int{3, 4, 5, 1, 2}
    min := findMin(nums)

    t.Log(nums)
    t.Log(min)

    nums = []int{4, 5, 6, 7, 0, 1, 2}
    min = findMin(nums)

    t.Log(nums)
    t.Log(min)
}
