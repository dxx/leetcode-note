package com.mcx;

/**
 * Created by mcx on 2020-08-30.
 *
 * 最小覆盖子串
 * 给你一个字符串 S、一个字符串 T 。请你设计一种算法，可以在 O(n) 的时间复杂度内，
 * 从字符串 S 里面找出：包含 T 所有字符的最小子串。
 *
 * 示例
 * 输入: S = "ADOBECODEBANC", T = "ABC"
 * 输出: "BANC"
 */
public class MinimumWindowSubstring {

    public static String minWindow(String s, String t) {
        if (t == null || t.equals("")) {
            return "";
        }

        // 存储字符和对应出现的次数, ascii 码有 127 个字符
        int[] sMap = new int[128], tMap = new int[128];

        for (int i = 0; i < t.length(); i++) {
            tMap[t.charAt(i)]++;
        }

        // 记录返回结果的左右下标
        int l = -1, r = -1;
        int minWindowLen = Integer.MAX_VALUE;

        // 记录每次滑动包含的字符个数
        int count = 0;

        // 左指针
        int left = 0;
        // 右指针
        int right = 0;
        while (right < s.length()) {
            char c = s.charAt(right);
            sMap[c]++;
            // 包含一个字符
            if (tMap[c] >= sMap[c]) {
                count++;
            }

            while (count == t.length()) {
                c = s.charAt(left);
                // 减少一个字符个数
                if (tMap[c] >= sMap[c]) {
                    count--;
                }
                // 重新给左右下标赋值
                if (right - left + 1 < minWindowLen) {
                    minWindowLen = right - left + 1;
                    l = left;
                    r = right + 1;
                }
                // 移除字符出现的次数
                sMap[c]--;
                left++;
            }

            right++;
        }
        if (l == -1) {
            return "";
        }
        return s.substring(l, r);
    }

    public static void main(String[] args) {
        String s = "ADOBECODEBANC";
        String t = "ABC";
        String r = minWindow(s, t);
        System.out.printf("s=%s, t=%s\n", s, t);
        System.out.println(r);
    }
}
