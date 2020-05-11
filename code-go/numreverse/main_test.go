package numreverse

import "testing"

func TestReverse(t *testing.T) {
    num := 123
    result := reverse(num)
    t.Logf("%d\n", result)

    num = -123
    result = reverse(num)
    t.Logf("%d\n", result)

    num = 120
    result = reverse(num)
    t.Logf("%d\n", result)

    num = 2147483647
    result = reverse(num)
    t.Logf("%d\n", result)
}
