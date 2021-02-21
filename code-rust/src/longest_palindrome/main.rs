/// 最长回文子串
/// 给定一个字符串 s，找到 s 中最长的回文子串。假设 s 的最大长度为 1000。

/// 示例
/// 输入: "babad"
/// 输出: "bab"
/// 注意: "aba" 也是一个有效答案。

/// 输入: "cbbd"
/// 输出: "bb"

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // 最长回文子字符串
        let mut palindrome_substring = "";
        let str = s.as_str();
        // 原字符串长度
        let str_len = str.len();
        // 记录子串的起始和下标以及对应是否为回文串
        let mut p_array = vec![vec![false; str_len]; str_len];

        // 暴力匹配
        // 时间复杂度o(n^3) 空间复杂度o(1)
        /*for i in 0..str_len {
            for j in i + 1..=str_len {
                let sub_str = &str[i..j];
                if Solution::is_palindrome(sub_str.to_string())
                    && sub_str.len() > palindrome_substring.len() {
                    palindrome_substring = sub_str;
                }
            }
        }*/

        // 动态规划
        // 时间复杂度o(n^2)，空间复杂度o(n^2)
        for i in 1..=str_len {
            for j in 0..str_len {
                let start = j;
                let end = start + i - 1;
                if end >= str_len {
                    break;
                }
                // (i == 1 || i == 2) && s[start] == s[end] 当只有一个字符或者两个字符时，直接判断
                // 如果 s[start+1, end-1] 是回文串并且 str[start] == s[end]
                p_array[start][end] = (i == 1 || i == 2 || p_array[start + 1][end - 1]) &&
                    &str[start..start + 1] == &s[end..end + 1];
                // 是回文串，并且长度超过已经保存的回文串长度
                if p_array[start][end] && i > palindrome_substring.len() {
                    palindrome_substring = &str[start..end + 1];
                }
            }
        }

        palindrome_substring.to_string()
    }

    #[allow(dead_code)]
    fn is_palindrome(s: String) -> bool {
        let s_bytes = s.as_bytes();
        for i in 0..s_bytes.len() {
            if s_bytes[i] != s_bytes[s_bytes.len() - i - 1] {
                return false
            }
        }
        true
    }
}

#[test]
fn test_longest_palindrome() {
    let str = "babad".to_string();
    let palindrome = Solution::longest_palindrome(str);
    println!("{}", palindrome);

    let str = "cbbd".to_string();
    let palindrome = Solution::longest_palindrome(str);
    println!("{}", palindrome);
}