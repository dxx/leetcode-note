/// 整数反转
/// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
/// 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31, 2^31 − 1]。
/// 请根据这个假设，如果反转后整数溢出那么就返回 0。

/// 示例
/// 输入: 123
/// 输出: 321

/// 输入: -123
/// 输出: -321

/// 输入: 120
/// 输出: 21

pub struct Solution {}

impl Solution {
    pub fn reverse(num: i32) -> i32 {
        let mut mut_num = num;
        let mut reverse_num: i32 = 0;
        while mut_num != 0 {
            // 计算出每一位的数字
            let digit = mut_num % 10;
            // 每次计算出一位数字后除以 10
            mut_num /= 10;
            // 将反转后的数字增加 10 倍
            if let Some(rev_num) = reverse_num.checked_mul(10) {
                // 把反转后的数字上累加到后面
                reverse_num = rev_num + digit;
            } else {
                // 越界，直接返回
                return 0;
            }
        }
        reverse_num
    }
}

#[test]
fn test_reverse() {
    let num = 123;
    let result = Solution::reverse(num);
    println!("{}", result);

    let num = -123;
    let result = Solution::reverse(num);
    println!("{}", result);

    let num = 120;
    let result = Solution::reverse(num);
    println!("{}", result);

    let num = 2147483647;
    let result = Solution::reverse(num);
    println!("{}", result);
}
