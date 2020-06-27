package twosum

import "testing"

func TestTwoSum2(t *testing.T) {
    numbers := []int{2, 7, 11, 15}
    target := 9
    result := twoSum2(numbers, target)

    t.Logf("numbers = %v, target = %d\n", numbers, target)
    t.Log(result)
}
