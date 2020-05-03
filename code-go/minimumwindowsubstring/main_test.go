package minimumwindowsubstring

import "testing"

func TestMinWindow(t *testing.T) {
    s := "ADOBECODEBANC"
    _t := "ABC"
    r := minWindow(s, _t)
    t.Logf("s=%s, t=%s\n", s, _t)
    t.Log(r)
}
