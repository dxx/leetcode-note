package palindromenum

import "testing"

func TestIsPalindrome(t *testing.T) {
    num := 121
    result := isPalindrome(num)
    t.Logf("%t\n", result)

    num = -121
    result = isPalindrome(num)
    t.Logf("%t\n", result)

    num = 10
    result = isPalindrome(num)
    t.Logf("%t\n", result)
}
