/// 整数拆分
/// 给定一个正整数 n，将其拆分为至少两个正整数的和，并使这些整数的乘积最大化。返回你可以获得的最大乘积。

/// 示例
/// 输入: 2
/// 输出: 1
/// 解释: 2 = 1 + 1, 1 × 1 = 1。

/// 输入: 10
/// 输出: 36
/// 解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。

pub struct Solution {}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        for i in 2..=n as usize {
            for j in 1..i {
                // 求拆分成 j 和 i - j 或者 j 和 i - j (能继续拆分)
                // 和上一次拆分 dp[i] (也就是拆分成 1, j - 1 ) 中的最大值
                dp[i] = dp[i].max((j as i32 * (i - j) as i32).max(j as i32 * dp[i - j]));
            }
        }
        dp[n as usize]
    }
}

#[test]
fn test_integer_break() {
    let n = 2;
    println!("{}", n);

    let max = Solution::integer_break(n);
    println!("{}", max);

    let n = 10;
    println!("{}", n);

    let max = Solution::integer_break(n);
    println!("{}", max);
}
