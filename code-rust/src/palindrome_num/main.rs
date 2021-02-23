/// 回文数
/// 判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

/// 示例
/// 输入: 121
/// 输出: true

/// 输入: -121
/// 输出: false
/// 解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。

/// 输入: 10
/// 输出: false
/// 解释: 从右向左读, 为 01 。因此它不是一个回文数。

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(num: i32) -> bool {
        if num < 0 {
            return false;
        }
        // 以下代码反转整数，不考虑越界
        let mut rev_num = 0;
        let mut n = num;
        while n != 0 {
            let digit = n % 10;
            n /= 10;
            rev_num = rev_num * 10 + digit;
        }
        num == rev_num
    }
}

#[test]
fn test_is_palindrome() {
    let num = 121;
    let result = Solution::is_palindrome(num);
    println!("{}", result);

    let num = -121;
    let result = Solution::is_palindrome(num);
    println!("{}", result);

    let num = 10;
    let result = Solution::is_palindrome(num);
    println!("{}", result);
}
