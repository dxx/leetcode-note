package convertsortedarraytobinarysearchtree

import (
    "fmt"
    "testing"
)

func TestSortedArrayToBST(t *testing.T) {
    nums := []int{-10, -3, 0, 5, 9}

    fmt.Println(nums)

    tree := sortedArrayToBST(nums)

    infixOrder(tree)
}
