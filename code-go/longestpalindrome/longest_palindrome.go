package main

import (
    "fmt"
)

// 最长回文子串
// 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

// 示例
// 输入: "babad"
// 输出: "bab"
// 注意: "aba" 也是一个有效答案。

// 输入: "cbbd"
// 输出: "bb"

func longestPalindrome(s string) string {
    // 最长回文子字符串
    palindromeSubstring := ""
    // 原字符串长度
    strLen := len(s)
    // 记录子串的起始和下标已经对应是否为回文串
    pArray := make([][]bool, strLen)
    for i := 0; i < len(pArray); i++ {
        pArray[i] = make([]bool, strLen)
    }

    //for i := 0; i < strLen; i++ {
    //    for j := i + 1; j < strLen; j++ {
    //        subStr := s[i:j+1]
    //        if isPalindrome(subStr) {
    //           // 判断是否需要修改最大回文子字符串
    //           if len(palindromeSubstring) < len(subStr) {
    //               palindromeSubstring = subStr
    //           }
    //        }
    //    }
    //}
    for i := 1; i <= strLen; i++ {
       for j := 0; j < strLen; j++ {
           start := j
           end := start + i - 1
           if end >= strLen {
               break
           }
           if (i == 1 || i == 2) && s[start] == s[end] {
               pArray[start][end] = true
           } else if pArray[start + 1][end - 1] && s[start] == s[end] {
               pArray[start][end] = true
           }
           if pArray[start][end] {
               palindromeSubstring = s[start:end+1]
           }
       }
    }

    return palindromeSubstring
}

func isPalindrome(s string) bool {
    for i := 0; i < len(s) / 2; i++ {
        if s[i] != s[len(s) - i - 1] {
            return false
        }
    }
    return true
}

func main() {
    str := "babad"
    palindrome := longestPalindrome(str)
    fmt.Println(palindrome)

    str = "cbbd"
    palindrome = longestPalindrome(str)
    fmt.Println(palindrome)
}
