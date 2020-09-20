package main

import "fmt"

// 单词拆分
// 给定一个非空字符串 s 和一个包含非空单词的列表 wordDict，判定 s 是否可以被空格拆分为一个或多个在字典中出现的单词。

// 说明:
//     分时可以重复使用字典中的单词
//     你可以假设字典中没有重复的单词

// 示例
// 输入: s = "leetcode", wordDict = ["leet", "code"]
// 输出: true
// 解释: 返回 true 因为 "leetcode" 可以被拆分成 "leet code"。

// 输入: s = "applepenapple", wordDict = ["apple", "pen"]
// 输出: true
// 解释: 返回 true 因为 "applepenapple" 可以被拆分成 "apple pen apple"。
//      注意你可以重复使用字典中的单词。

// 输入: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
// 输出: false

func wordBreak(s string, wordDict []string) bool {
    wordDictSet := make(map[string]bool)
    // 将字典数组放入哈希表
    for _, str := range wordDict {
        wordDictSet[str] = true
    }
    sLen := len(s)
    // 存储前 i 个字符组成的字符串是否能被空格拆分成若干个字典中出现的单词
    dp := make([]bool, sLen + 1)
    // 表示空串能够拆分
    dp[0] = true
    for i := 1; i <= sLen; i++ {
        // 判断前 i 个字符组成的字符串是否能被空格拆分成若干个字典中出现的单词
        for j := 0; j < i; j++ {
            // 判断前 j 个字符组成的字符串和剩余的 s[j, i - 1] 是否在字典中出现
            if dp[j] && wordDictSet[s[j:i]] {
                dp[i] = true
                // 只要可以被拆分, 停止继续搜索
                break
            }
        }
    }
    return dp[sLen]
}

func main() {
    s := "leetcode"
    wordDict := []string{"leet", "code"}
    fmt.Printf("s=%s, wordDict=%v\n", s, wordDict)

    result := wordBreak(s, wordDict)
    fmt.Println(result)

    s = "applepenapple"
    wordDict = []string{"apple", "pen"}
    fmt.Printf("s=%s, wordDict=%v\n", s, wordDict)

    result = wordBreak(s, wordDict)
    fmt.Println(result)

    s = "catsandog"
    wordDict = []string{"cats", "dog", "sand", "and", "cat"}
    fmt.Printf("s=%s, wordDict=%v\n", s, wordDict)

    result = wordBreak(s, wordDict)
    fmt.Println(result)
}
