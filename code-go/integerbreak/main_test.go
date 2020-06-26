package integerbreak

import "testing"

func TestIntegerBreak(t *testing.T) {
	n := 2
	t.Log(n)

	max := integerBreak(n)
	t.Log(max)

	n = 10
	t.Log(n)

	max = integerBreak(n)
	t.Log(max)
}
