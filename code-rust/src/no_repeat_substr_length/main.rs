use std::collections::HashSet;

/// 无重复字符的最长子串
/// 给定一个字符串，找出其中不含有重复字符的 **最长子串** 和 **长度**。

/// 示例
/// 输入: "abcabcbb"
/// 输出: 3
/// 解释: 无重复字符的最长子串是 "abc"，其长度为 3。

/// 输入: "bbbbb"
/// 输出: 1
/// 解释: 无重复字符的最长子串是 "b"，其长度为 1。

/// 输入: "pwwkew"
/// 输出: 3
/// 解释: 无重复字符的最长子串是 "wke"，其长度为 3。
///     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

pub struct Solution {}

impl Solution {
    pub fn longest_substring(s: String) -> String {
        // 记录重复字符
        let mut str_set = HashSet::new();
        // 最长无重复的子字符串
        let mut max_substring = "";
        let bytes = s.as_str();
        // 原字符串长度
        let str_len = s.len();
        // 右下标
        let mut j = 0;

        for i in 0..str_len {
            if j >= str_len {
                break;
            }
            // 右下标不断后移，直到发现重复的字符
            while j < str_len && !str_set.contains(&bytes[j..j + 1]) {
                str_set.insert(&s[j..j + 1]);
                j += 1;
            }
            // 判断是否需要修改最大子字符串
            if (max_substring.len() as i32) < j as i32 - i as i32 {
                max_substring = &bytes[i..j];
            }
            // 下一次循环 i 要右移一位，删除左边第一个字符
            str_set.remove(&bytes[i..i + 1]);
        }
        max_substring.to_string()
    }
}

#[test]
fn test_longest_substring() {
    let str = "abcabcbb".to_string();
    let max_substring = Solution::longest_substring(str.clone());
    println!(
        "字符串 {} 的无重复字符的最长子串是 {} 长度为 {}",
        str,
        max_substring,
        max_substring.len()
    );

    let str = "bbbbb".to_string();
    let max_substring = Solution::longest_substring(str.clone());
    println!(
        "字符串 {} 的无重复字符的最长子串是 {} 长度为 {}",
        str,
        max_substring,
        max_substring.len()
    );

    let str = "pwwkew".to_string();
    let max_substring = Solution::longest_substring(str.clone());
    println!(
        "字符串 {} 的无重复字符的最长子串是 {} 长度为 {}",
        str,
        max_substring,
        max_substring.len()
    );
}
