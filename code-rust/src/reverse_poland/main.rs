/// 逆波兰表达式求值
/// 根据 逆波兰表示法，求表达式的值。
/// 有效的运算符包括 +, -, *, / 。每个运算对象可以是整数，也可以是另一个逆波兰表达式。

/// 逆波兰表达式是一种后缀表达式，所谓后缀就是指运算符写在后面。
/// 平常使用的算式则是一种中缀表达式，如 ( 1 + 2 ) * ( 3 + 4 ) 。
/// 该算式的逆波兰表达式写法为 ( ( 1 2 + ) ( 3 4 + ) * ) 。

/// 说明：
/// 整数除法只保留整数部分。
/// 给定逆波兰表达式总是有效的。换句话说，表达式总会得出有效数值且不存在除数为 0 的情况。

/// 示例
/// 输入: ["2", "1", "+", "3", "*"]
/// 输出: 9
/// 解释: 该算式转化为常见的中缀算术表达式为：((2 + 1) * 3) = 9

/// 输入: ["4", "13", "5", "/", "+"]
/// 输出: 6
/// 解释: 该算式转化为常见的中缀算术表达式为：(4 + (13 / 5)) = 6

/// 输入: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
/// 输出: 22
/// 解释:
/// 该算式转化为常见的中缀算术表达式为：
/// ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
/// = ((10 * (6 / (12 * -11))) + 17) + 5
/// = ((10 * (6 / -132)) + 17) + 5
/// = ((10 * 0) + 17) + 5
/// = (0 + 17) + 5
/// = 17 + 5
/// = 22

#[derive(Clone, Debug)]
pub struct Operation {
    opt: String,
    opt_func: fn(i32, i32) -> i32,
}

pub struct ReversePoland {
    operations: std::collections::HashMap<String, Operation>,
}

impl ReversePoland {
    pub fn new() -> Self {
        return ReversePoland {
            operations: [
                (
                    String::from("+"),
                    Operation {
                        opt: String::from("+"),
                        opt_func: |num1: i32, num2: i32| -> i32 { num1 + num2 },
                    },
                ),
                (
                    String::from("-"),
                    Operation {
                        opt: String::from("-"),
                        opt_func: |num1: i32, num2: i32| -> i32 { num1 - num2 },
                    },
                ),
                (
                    String::from("*"),
                    Operation {
                        opt: String::from("*"),
                        opt_func: |num1: i32, num2: i32| -> i32 { num1 * num2 },
                    },
                ),
                (
                    String::from("/"),
                    Operation {
                        opt: String::from("/"),
                        opt_func: |num1: i32, num2: i32| -> i32 { num1 / num2 },
                    },
                ),
            ]
                .iter()
                .cloned()
                .collect(),
        };
    }

    /// 计算结果
    fn calculate_num(&self, mut num1: i32, mut num2: i32, opt: &str) -> i32 {
        let opt_func = self.operations.get(opt).unwrap().opt_func;
        if opt == "-" || opt == "/" {
            // 因为出栈后两数的位置颠倒，需交换两个数的位置
            num1 = num1 + num2;
            num2 = num1 - num2;
            num1 = num1 - num2;
        }
        return opt_func(num1, num2);
    }
}

pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let reverse_poland = ReversePoland::new();
        let mut num_stacks = Vec::new();
        for token in tokens.into_iter() {
            if let Some(_) = reverse_poland.operations.get(&token) {
                // 取出两个数计算
                let num1 = num_stacks.pop().unwrap();
                let num2 = num_stacks.pop().unwrap();
                let result = reverse_poland.calculate_num(num1, num2, &token);
                num_stacks.push(result);
                continue
            }
            num_stacks.push(token.parse::<i32>().unwrap());
        }
        // 栈中最后一个元素就是最后的结果
        *num_stacks.get(0).unwrap()
    }
}

#[test]
fn test_eval_rpn() {
    let tokens: Vec<String> = vec!["2".to_string(), "1".to_string(),
        "+".to_string(), "3".to_string(), "*".to_string(),
    ];
    let res = Solution::eval_rpn(tokens.clone());

    println!("{:?}", tokens);
    println!("{}", res);

    let tokens: Vec<String> = vec![
        "4".to_string(), "13".to_string(), "5".to_string(),
        "/".to_string(), "+".to_string(),
    ];
    let res = Solution::eval_rpn(tokens.clone());

    println!("{:?}", tokens);
    println!("{}", res);

    let tokens: Vec<String> = vec![
        "10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(),
        "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), "17".to_string(),
        "+".to_string(), "5".to_string(), "+".to_string(),
    ];
    let res = Solution::eval_rpn(tokens.clone());

    println!("{:?}", tokens);
    println!("{}", res);
}
