/// 最小覆盖子串
/// 给你一个字符串 S、一个字符串 T 。请你设计一种算法，可以在 O(n) 的时间复杂度内，
/// 从字符串 S 里面找出：包含 T 所有字符的最小子串。

/// 示例
/// 输入: S = "ADOBECODEBANC", T = "ABC"
/// 输出: "BANC"

pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        if t_bytes == "".as_bytes() {
            return String::new();
        }
        // 存储字符和对应出现的次数, ascii 码有 127 个字符
        let (mut s_map, mut t_map) = ([0; 128], [0; 128]);

        for i in 0..t_bytes.len() {
            t_map[t_bytes[i] as usize] += 1;
        }

        // 记录返回结果的左右下标
        let (mut l, mut r) = (-1, -1);
        let mut min_window_len = i32::MAX;

        // 记录每次滑动包含的字符个数
        let mut count = 0;
        let (mut left, mut right) = (0, 0);
        while right < s_bytes.len() {
            s_map[s_bytes[right] as usize] += 1;
            // 包含一个字符
            if t_map[s_bytes[right] as usize] >= s_map[s_bytes[right] as usize] {
                count += 1;
            }

            while count == t.len() {
                // 减少一个字符个数
                if t_map[s_bytes[left] as usize] >= s_map[s_bytes[left] as usize] {
                    count -= 1;
                }
                // 重新给左右下标赋值
                if right - left + 1 < min_window_len as usize {
                    min_window_len = right as i32 - left as i32 + 1;
                    l = left as i32;
                    r = right as i32 + 1;
                }
                // 移除字符出现的次数
                s_map[s_bytes[left] as usize] -= 1;
                left += 1;
            }
            right += 1;
        }
        if l == -1 {
            return "".to_string();
        }
        return s[l as usize..r as usize].to_string();
    }
}

#[test]
fn test_min_window() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    println!("s={}, t={}", s, t);
    let r = Solution::min_window(s, t);
    println!("{}", r);
}
