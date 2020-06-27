package partitionequalsubsetsum

import "testing"

func TestCanPartition(t *testing.T) {
	nums := []int{1, 5, 11, 5}
	t.Log(nums)

	result := canPartition(nums)
	t.Log(result)

	nums = []int{1, 2, 3, 5}
	t.Log(nums)

	result = canPartition(nums)
	t.Log(result)
}
