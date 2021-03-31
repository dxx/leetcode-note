/// 单词拆分
/// 给定一个非空字符串 s 和一个包含非空单词的列表 wordDict，判定 s 是否可以被空格拆分为一个或多个在字典中出现的单词。

/// 说明:
///     分时可以重复使用字典中的单词
///     你可以假设字典中没有重复的单词

/// 示例
/// 输入: s = "leetcode", wordDict = ["leet", "code"]
/// 输出: true
/// 解释: 返回 true 因为 "leetcode" 可以被拆分成 "leet code"。

/// 输入: s = "applepenapple", wordDict = ["apple", "pen"]
/// 输出: true
/// 解释: 返回 true 因为 "applepenapple" 可以被拆分成 "apple pen apple"。
///     注意你可以重复使用字典中的单词。

/// 输入: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
/// 输出: false

pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let str = s.as_str();
        let mut word_dict_set = std::collections::HashMap::new();
        // 将字典数组放入哈希表
        for str in word_dict.into_iter() {
            word_dict_set.insert(str, true);
        }
        let s_len = str.len();
        // 存储前 i 个字符组成的字符串是否能被空格拆分成若干个字典中出现的单词
        let mut dp = vec![false; s_len + 1];
        // 表示空串能够拆分
        dp[0] = true;
        for i in 1..=s_len {
            // 判断前 i 个字符组成的字符串是否能被空格拆分成若干个字典中出现的单词
            for j in 0..i {
                // 判断前 j 个字符组成的字符串和剩余的 s[j, i - 1] 是否在字典中出现
                if let Some(exist) = word_dict_set.get(&str[j..i]) {
                    if dp[j] && *exist {
                        dp[i] = true;
                        // 只要可以被拆分, 停止继续搜索
                        break;
                    }
                }
            }
        }
        dp[s_len]
    }
}

#[test]
fn test_word_break() {
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    println!("s={}, wordDict={:?}", s, word_dict);

    let result = Solution::word_break(s, word_dict);
    println!("{}", result);

    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    println!("s={}, wordDict={:?}", s, word_dict);

    let result = Solution::word_break(s, word_dict);
    println!("{}", result);

    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    println!("s={}, wordDict={:?}", s, word_dict);

    let result = Solution::word_break(s, word_dict);
    println!("{}", result);
}
