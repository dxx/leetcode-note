package minimumpathsum

import "testing"

func TestMinPathSum(t *testing.T) {
    grid := [][]int {
        {1, 3, 1},
        {1, 5, 1},
        {4, 2, 1},
    }
    sum := minPathSum(grid)
    t.Log(sum)
}
