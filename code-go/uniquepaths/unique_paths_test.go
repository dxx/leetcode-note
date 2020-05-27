package uniquepaths

import "testing"

func TestUniquePaths(t *testing.T) {
    m := 3
    n := 2
    t.Logf("m = %d, n = %d\n", m, n)
    count := uniquePaths(m, n)
    t.Log(count)

    m = 7
    n = 3
    t.Logf("m = %d, n = %d\n", m, n)
    count = uniquePaths(m, n)
    t.Log(count)
}
