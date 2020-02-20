package com.mcx.longestpalindrome;

/**
 * 最长回文子串
 * 给定一个字符串 s，找到 s 中最长的回文子串。假设 s 的最大长度为 1000。
 *
 * 示例
 * 输入: "babad"
 * 输出: "bab"
 * 注意: "aba" 也是一个有效答案。
 *
 * 输入: "cbbd"
 * 输出: "bb"
 */
public class Main {

    public static String longestPalindrome(String s) {
        // 最长回文子字符串
        String palindromeSubstring = "";
        // 原字符串长度
        int strLen = s.length();
        // 记录子串的起始和下标以及对应是否为回文串
        boolean[][] pArray = new boolean[strLen][strLen];

        // 暴力匹配
        // 时间复杂度o(n^3) 空间复杂度o(1)
//        for (int i = 0; i < strLen; i++) {
//            for (int j = i + 1; j <= strLen; j++) {
//                String subStr = s.substring(i, j);
//                if (isPalindrome(subStr) && subStr.length() > palindromeSubstring.length()) {
//                    palindromeSubstring = subStr;
//                }
//            }
//        }

        // 动态规划
        // 时间复杂度o(n^2)，空间复杂度o(n^2)
        for (int i = 1; i <= strLen; i++) {
            for (int start = 0; start < strLen; start++) {
                int end = start + i - 1;
                if (end >= strLen) {
                    break;
                }
                pArray[start][end] = (i == 1 || i == 2 || pArray[start + 1][end - 1])
                        && s.charAt(start) == s.charAt(end);
                // 是回文串，并且长度超过已经保存的回文串长度
                if (pArray[start][end] && i > palindromeSubstring.length()) {
                    palindromeSubstring = s.substring(start, end + 1);
                }
            }
        }

        return palindromeSubstring;
    }

    public static boolean isPalindrome(String s) {
        for (int i = 0; i < s.length() / 2; i++) {
            if (s.charAt(i) != s.charAt(s.length() - i - 1)) {
                return false;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        String str = "babad";
        String palindrome = longestPalindrome(str);
        System.out.println(palindrome);

        str = "cbbd";
        palindrome = longestPalindrome(str);
        System.out.println(palindrome);
    }
}
