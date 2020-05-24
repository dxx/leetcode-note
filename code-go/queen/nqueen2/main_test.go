package nqueen2

import "testing"

func TestTotalNQueens(t *testing.T) {
    n := 4
    total := totalNQueens(n)
    t.Log(n)
    t.Logf("%v\n", total)
}
