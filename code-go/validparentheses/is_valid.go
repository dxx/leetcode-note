package main

import "fmt"

// 有效的括号
// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
// 有效字符串需满足：
//    左括号必须用相同类型的右括号闭合。
//    左括号必须以正确的顺序闭合。
// 注意空字符串可被认为是有效字符串。

// 示例
// 输入: "()"
// 输出: true

// 输入: "()[]{}"
// 输出: true

// 输入: "(]"
// 输出: false

// 输入: "([)]"
// 输出: false

// 输入: "{[]}"
// 输出: true

func isValid(s string) bool {
    stack := make([]byte, 0)
    for i := 0; i < len(s); i++ {
        c := s[i]
        // 将（ { [ 入栈
        if c == '(' || c == '{' || c == '[' {
            stack = append(stack, c)
            continue
        }
        // 如果栈为空,表示 ( { [ 中没有一个符号入栈
        if len(stack) == 0 {
            return false
        }
        char := stack[len(stack) - 1]
        stack = stack[0:len(stack) - 1]
        switch c {
            case ')':
                // 判断是否和栈顶元素配对
                if char != '(' {
                    return false
                }
            case '}':
                if char != '{' {
                    return false
                }
            case ']':
                if char != '[' {
                    return false
                }
        }
    }
    // 所有的括号能配对, 栈就会为空
    return len(stack) == 0
}

func main() {
    s := "()"
    fmt.Println(s)

    r := isValid(s)
    fmt.Println(r)

    s = "()[]{}"
    fmt.Println(s)

    r = isValid(s)
    fmt.Println(r)

    s = "(]"
    fmt.Println(s)

    r = isValid(s)
    fmt.Println(r)

    s = "([)]"
    fmt.Println(s)

    r = isValid(s)
    fmt.Println(r)

    s = "{[]}"
    fmt.Println(s)

    r = isValid(s)
    fmt.Println(r)
}
