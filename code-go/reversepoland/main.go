package reversepoland

import (
    "fmt"
    "strconv"
)

// 逆波兰表达式求值
// 根据 逆波兰表示法，求表达式的值。
// 有效的运算符包括 +, -, *, / 。每个运算对象可以是整数，也可以是另一个逆波兰表达式。

// 逆波兰表达式是一种后缀表达式，所谓后缀就是指运算符写在后面。
// 平常使用的算式则是一种中缀表达式，如 ( 1 + 2 ) * ( 3 + 4 ) 。
// 该算式的逆波兰表达式写法为 ( ( 1 2 + ) ( 3 4 + ) * ) 。

// 说明：
// 整数除法只保留整数部分。
// 给定逆波兰表达式总是有效的。换句话说，表达式总会得出有效数值且不存在除数为 0 的情况。

// 示例
// 输入: ["2", "1", "+", "3", "*"]
// 输出: 9
// 解释: 该算式转化为常见的中缀算术表达式为：((2 + 1) * 3) = 9

// 输入: ["4", "13", "5", "/", "+"]
// 输出: 6
// 解释: 该算式转化为常见的中缀算术表达式为：(4 + (13 / 5)) = 6

// 输入: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
// 输出: 22
// 解释:
// 该算式转化为常见的中缀算术表达式为：
// ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
// = ((10 * (6 / (12 * -11))) + 17) + 5
// = ((10 * (6 / -132)) + 17) + 5
// = ((10 * 0) + 17) + 5
// = (0 + 17) + 5
// = 17 + 5
// = 22

type Operation struct {
    opt string
    optFunc   func(num1, num2 int) int
}

// 运算符和对应的计算方法集合
var operations = map[string]Operation {
    "+" : {"+", func(num1, num2 int)int { return num1 + num2 }},
    "-" : {"-", func(num1, num2 int)int { return num1 - num2 }},
    "*" : {"*", func(num1, num2 int)int { return num1 * num2 }},
    "/" : {"/", func(num1, num2 int)int { return num1 / num2 }},
}

// 计算值
func calculateNum(num1, num2 int, opt string) int {
    operation, ok := operations[opt]
    if !ok {
        panic("无效的运算符:" + opt)
    }
    if opt == "-" || opt == "/" {
        // 因为出栈后两个数颠倒，需要交换两个数
        num1, num2 = num2, num1
    }
    return operation.optFunc(num1, num2)
}

func evalRPN(tokens []string) int {
    // 模拟栈
    numStacks := make([]int, len(tokens))
    // 数组当前元素下标
    index := 0
    for _, token := range tokens {
        _, ok := operations[token]
        if ok {
            // 取出两个数计算
            num1 := numStacks[index - 1]
            num2 := numStacks[index - 2]
            result := calculateNum(num1, num2, token)
            numStacks[index - 2] = result
            index = index - 1
            continue
        }
        elem, _ := strconv.Atoi(token)
        numStacks[index] = elem
        index++
    }
    // 栈中最后一个元素就是最后的结果
    return numStacks[0]
}

func main() {
    tokens := []string{"2", "1", "+", "3", "*"}
    res := evalRPN(tokens)

    fmt.Printf("%v\n", tokens)
    fmt.Printf("%v\n", res)

    tokens = []string{"4", "13", "5", "/", "+"}
    res = evalRPN(tokens)

    fmt.Printf("%v\n", tokens)
    fmt.Printf("%v\n", res)

    tokens = []string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"}
    res = evalRPN(tokens)

    fmt.Printf("%v\n", tokens)
    fmt.Printf("%v\n", res)
}
