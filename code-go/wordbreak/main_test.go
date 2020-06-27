package wordbreak

import "testing"

func TestWordBreak(t *testing.T) {
    s := "leetcode"
    wordDict := []string{"leet", "code"}
    t.Logf("s=%s, wordDict=%v\n", s, wordDict)

    result := wordBreak(s, wordDict)
    t.Log(result)

    s = "applepenapple"
    wordDict = []string{"apple", "pen"}
    t.Logf("s=%s, wordDict=%v\n", s, wordDict)

    result = wordBreak(s, wordDict)
    t.Log(result)

    s = "catsandog"
    wordDict = []string{"cats", "dog", "sand", "and", "cat"}
    t.Logf("s=%s, wordDict=%v\n", s, wordDict)

    result = wordBreak(s, wordDict)
    t.Log(result)
}
