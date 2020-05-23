package insertposition

import "testing"

func TestSearchInsert(t *testing.T) {
    nums := []int{1, 3, 5, 6}

    target := 5
    index := searchInsert(nums, target)

    t.Logf("%v, %d\n", nums, target)
    t.Log(index)

    target = 2
    index = searchInsert(nums, target)

    t.Logf("%v, %d\n", nums, target)
    t.Log(index)

    target = 7
    index = searchInsert(nums, target)

    t.Logf("%v, %d\n", nums, target)
    t.Log(index)

    target = 0
    index = searchInsert(nums, target)

    t.Logf("%v, %d\n", nums, target)
    t.Log(index)
}
