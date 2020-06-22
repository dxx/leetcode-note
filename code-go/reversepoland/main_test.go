package reversepoland

import "testing"

func TestEvalRPN(t *testing.T) {
	tokens := []string{"2", "1", "+", "3", "*"}
	res := evalRPN(tokens)

	t.Logf("%v\n", tokens)
	t.Logf("%v\n", res)

	tokens = []string{"4", "13", "5", "/", "+"}
	res = evalRPN(tokens)

	t.Logf("%v\n", tokens)
	t.Logf("%v\n", res)

	tokens = []string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"}
	res = evalRPN(tokens)

	t.Logf("%v\n", tokens)
	t.Logf("%v\n", res)
}
