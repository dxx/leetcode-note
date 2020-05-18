package validparentheses

import "testing"

func TestIsValid(t *testing.T) {
    s := "()"
    t.Log(s)

    r := isValid(s)
    t.Log(r)

    s = "()[]{}"
    t.Log(s)

    r = isValid(s)
    t.Log(r)

    s = "(]"
    t.Log(s)

    r = isValid(s)
    t.Log(r)

    s = "([)]"
    t.Log(s)

    r = isValid(s)
    t.Log(r)

    s = "{[]}"
    t.Log(s)

    r = isValid(s)
    t.Log(r)
}
