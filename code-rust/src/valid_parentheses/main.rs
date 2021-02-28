/// 有效的括号
/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
/// 有效字符串需满足：
///    左括号必须用相同类型的右括号闭合。
///    左括号必须以正确的顺序闭合。
/// 注意空字符串可被认为是有效字符串。

/// 示例
/// 输入: "()"
/// 输出: true

/// 输入: "()[]{}"
/// 输出: true

/// 输入: "(]"
/// 输出: false

/// 输入: "([)]"
/// 输出: false

/// 输入: "{[]}"
/// 输出: true

pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            // 将（ { [ 入栈
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
                continue;
            }
            // 如果栈为空,表示 ( { [ 中没有一个符号入栈
            if stack.is_empty() {
                return false;
            }
            let ch = stack.pop().unwrap();
            match c {
                ')' => {
                    // 判断是否和栈顶元素配对
                    if ch != '(' {
                        return false;
                    }
                }
                '}' => {
                    if ch != '{' {
                        return false;
                    }
                }
                ']' => {
                    if ch != '[' {
                        return false;
                    }
                }
                _ => {}
            }
        }
        // 所有的括号能配对, 栈就会为空
        stack.is_empty()
    }
}

#[test]
fn test_is_valid() {
    let s = "()".to_string();
    println!("{}", s);

    let r = Solution::is_valid(s);
    println!("{}", r);

    let s = "()[]{}".to_string();
    println!("{}", s);

    let r = Solution::is_valid(s);
    println!("{}", r);

    let s = "(]".to_string();
    println!("{}", s);

    let r = Solution::is_valid(s);
    println!("{}", r);

    let s = "([)]".to_string();
    println!("{}", s);

    let r = Solution::is_valid(s);
    println!("{}", r);

    let s = "{[]}".to_string();
    println!("{}", s);

    let r = Solution::is_valid(s);
    println!("{}", r);
}
