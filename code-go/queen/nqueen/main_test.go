package nqueen

import "testing"

func TestSolveNQueens(t *testing.T) {
    n := 4
    outputs := solveNQueens(n)
    t.Log(n)
    t.Logf("%v\n", outputs)
}
