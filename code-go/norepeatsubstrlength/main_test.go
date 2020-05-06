package norepeatsubstrlength

import (
    "fmt"
    "testing"
)

func TestLongestSubstring(t *testing.T) {
    str := "abcabcbb"
    maxSubstring := longestSubstring(str)
    fmt.Printf("字符串 %s 的无重复字符的最长子串是 %s 长度为 %d\n", str, maxSubstring, len(maxSubstring))

    str = "bbbbb"
    maxSubstring = longestSubstring(str)
    fmt.Printf("字符串 %s 的无重复字符的最长子串是 %s 长度为 %d\n", str, maxSubstring, len(maxSubstring))

    str = "pwwkew"
    maxSubstring = longestSubstring(str)
    fmt.Printf("字符串 %s 的无重复字符的最长子串是 %s 长度为 %d\n", str, maxSubstring, len(maxSubstring))
}
