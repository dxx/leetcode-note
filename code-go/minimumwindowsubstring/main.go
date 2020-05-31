package minimumwindowsubstring

import (
    "math"
)

// 最小覆盖子串
// 给你一个字符串 S、一个字符串 T 。请你设计一种算法，可以在 O(n) 的时间复杂度内，
// 从字符串 S 里面找出：包含 T 所有字符的最小子串。

// 示例
// 输入: S = "ADOBECODEBANC", T = "ABC"
// 输出: "BANC"

func minWindow(s string, t string) string {
    if t == "" {
        return ""
    }
    // 存储字符和对应出现的次数, ascii 码有 127 个字符
    sMap, tMap := [128]int{}, [128]int{}

    for i := 0; i < len(t); i++ {
        tMap[t[i]]++
    }

    // 记录返回结果的左右下标
    l, r := -1, -1
    minWindowLen := math.MaxInt32

    // 记录每次滑动包含的字符个数
    count := 0

    // 左指针
    left := 0
    // 右指针
    right := 0
    for right < len(s) {
        sMap[s[right]]++
        // 包含一个字符
        if tMap[s[right]] >= sMap[s[right]] {
            count++
        }

        for count == len(t) {
            // 减少一个字符个数
            if tMap[s[left]] >= sMap[s[left]] {
                count--
            }
            // 重新给左右下标赋值
            if right - left + 1 < minWindowLen {
                minWindowLen = right - left + 1
                l = left
                r = right + 1
            }
            // 移除字符出现的次数
            sMap[s[left]]--
            left++
        }

        right++
    }
    if l == -1 {
        return ""
    }
    return s[l:r]
}
