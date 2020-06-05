package uniquebinarysearchtree

import (
    "fmt"
    "testing"
)

func TestGenerateTrees(t *testing.T) {
    n := 3

    fmt.Println(n)

    trees := generateTrees(n)
    for _, tree := range trees {
        preOrder(tree)
        fmt.Println()
    }
}
