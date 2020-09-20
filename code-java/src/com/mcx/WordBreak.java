package com.mcx;

import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * Created by mcx on 2020-09-20.
 *
 * 单词拆分
 * 给定一个非空字符串 s 和一个包含非空单词的列表 wordDict，判定 s 是否可以被空格拆分为一个或多个在字典中出现的单词。
 *
 * 说明:
 *     分时可以重复使用字典中的单词
 *     你可以假设字典中没有重复的单词
 *
 * 示例
 * 输入: s = "leetcode", wordDict = ["leet", "code"]
 * 输出: true
 * 解释: 返回 true 因为 "leetcode" 可以被拆分成 "leet code"。
 *
 * 输入: s = "applepenapple", wordDict = ["apple", "pen"]
 * 输出: true
 * 解释: 返回 true 因为 "applepenapple" 可以被拆分成 "apple pen apple"。
 *     注意你可以重复使用字典中的单词。
 *
 * 输入: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
 * 输出: false
 */
public class WordBreak {

    public static boolean wordBreak(String s, List<String> wordDict) {
        Set<String> wordDictSet = new HashSet<>(wordDict);
        int sLen = s.length();
        // 存储前 i 个字符组成的字符串是否能被空格拆分成若干个字典中出现的单词
        boolean[] dp = new boolean[sLen + 1];
        // 表示空串能够拆分
        dp[0] = true;
        for (int i = 1;i <= sLen; i++) {
            // 判断前 i 个字符组成的字符串是否能被空格拆分成若干个字典中出现的单词
            for (int j = 0; j < i; j++) {
                // 判断前 j 个字符组成的字符串和剩余的 s[j, i - 1] 是否在字典中出现
                if (dp[j] && wordDictSet.contains(s.substring(j, i))) {
                    dp[i] = true;
                    // 只要可以被拆分, 停止继续搜索
                    break;
                }
            }
        }
        return dp[sLen];
    }

    public static void main(String[] args) {
        String s = "leetcode";
        List<String> wordDict = Arrays.asList("leet", "code");
        StringBuilder sb = new StringBuilder("[");
        for (String word : wordDict) {
            sb.append(word).append(",").append(" ");
        }
        sb.deleteCharAt(sb.length() - 1);
        sb.deleteCharAt(sb.length() - 1);
        sb.append("]");
        System.out.printf("s=%s, wordDict=%s\n", s, sb);

        boolean result = wordBreak(s, wordDict);
        System.out.println(result);

        s = "applepenapple";
        wordDict = Arrays.asList("apple", "pen");
        sb = new StringBuilder("[");
        for (String word : wordDict) {
            sb.append(word).append(",").append(" ");
        }
        sb.deleteCharAt(sb.length() - 1);
        sb.deleteCharAt(sb.length() - 1);
        sb.append("]");
        System.out.printf("s=%s, wordDict=%s\n", s, sb);

        result = wordBreak(s, wordDict);
        System.out.println(result);

        s = "catsandog";
        wordDict = Arrays.asList("cats", "dog", "sand", "and", "cat");
        sb = new StringBuilder("[");
        for (String word : wordDict) {
            sb.append(word).append(",").append(" ");
        }
        sb.deleteCharAt(sb.length() - 1);
        sb.deleteCharAt(sb.length() - 1);
        sb.append("]");
        System.out.printf("s=%s, wordDict=%s\n", s, sb);

        result = wordBreak(s, wordDict);
        System.out.println(result);
    }
}
