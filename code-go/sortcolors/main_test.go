package sortcolors

import "testing"

func TestSortColors(t *testing.T) {
    nums := []int{2, 0, 2, 1, 1, 0}
    t.Log(nums)

    sortColors(nums)
    t.Log(nums)
}
