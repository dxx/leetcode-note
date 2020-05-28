package uniquepaths

import "testing"

func TestUniquePathsWithObstacles(t *testing.T) {
    obstacleGrid := [][]int {
        {0, 0, 0},
        {0, 1, 0},
        {0, 0, 0},
    }
    count := uniquePathsWithObstacles(obstacleGrid)
    t.Log(count)
}