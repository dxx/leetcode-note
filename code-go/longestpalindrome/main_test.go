package longestpalindrome

import "testing"

func TestLongestPalindrome(t *testing.T) {
    str := "babad"
    palindrome := longestPalindrome(str)
    t.Log(palindrome)

    str = "cbbd"
    palindrome = longestPalindrome(str)
    t.Log(palindrome)
}
