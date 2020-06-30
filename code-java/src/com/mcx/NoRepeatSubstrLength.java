package com.mcx;

import java.util.HashSet;
import java.util.Set;

/**
 * Created by mcx on 2020-06-30.
 *
 * 无重复字符的最长子串
 * 给定一个字符串，找出其中不含有重复字符的 **最长子串** 和 **长度**。
 *
 * 示例
 * 输入: "abcabcbb"
 * 输出: 3
 * 解释: 无重复字符的最长子串是 "abc"，其长度为 3。
 *
 * 输入: "bbbbb"
 * 输出: 1
 * 解释: 无重复字符的最长子串是 "b"，其长度为 1。
 *
 * 输入: "pwwkew"
 * 输出: 3
 * 解释: 无重复字符的最长子串是 "wke"，其长度为 3。
 *      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 */
public class NoRepeatSubstrLength {

    public static String longestSubstring(String s) {
        // 记录重复字符
        Set<Character> strSet = new HashSet<>();
        // 最长无重复的子字符串
        String maxSubstring = "";
        // 原字符串长度
        int strLen = s.length();
        // 右下标
        int j = 0;

        for (int i = 0; i < strLen; i++) {
            if (j >= strLen) {
                break;
            }

            // 右下标不断后移，直到发现重复的字符
            while (j < strLen && !strSet.contains(s.charAt(j))) {
                strSet.add(s.charAt(j));
                j++;
            }
            // 判断是否需要修改最大子字符串
            if (maxSubstring.length() < j - i) {
                maxSubstring = s.substring(i, j);
            }
            // 下一次循环 i 要右移一位，删除左边第一个字符
            strSet.remove(s.charAt(i));
        }

        return maxSubstring;
    }

    public static void main(String[] args) {
        String str = "abcabcbb";
        String maxSubstring = longestSubstring(str);
        System.out.printf("字符串 %s 的无重复字符的最长子串是 %s 长度为 %d\n", str, maxSubstring, maxSubstring.length());

        str = "bbbbb";
        maxSubstring = longestSubstring(str);
        System.out.printf("字符串 %s 的无重复字符的最长子串是 %s 长度为 %d\n", str, maxSubstring, maxSubstring.length());

        str = "pwwkew";
        maxSubstring = longestSubstring(str);
        System.out.printf("字符串 %s 的无重复字符的最长子串是 %s 长度为 %d\n", str, maxSubstring, maxSubstring.length());
    }
}
