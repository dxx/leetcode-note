package com.mcx.reversepoland;

import com.mcx.ReversePoland;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

/**
 * 逆波兰表达式求值
 * 根据 逆波兰表示法，求表达式的值。
 * 有效的运算符包括 +, -, *, / 。每个运算对象可以是整数，也可以是另一个逆波兰表达式。
 *
 * 逆波兰表达式是一种后缀表达式，所谓后缀就是指运算符写在后面。
 * 平常使用的算式则是一种中缀表达式，如 ( 1 + 2 ) * ( 3 + 4 ) 。
 * 该算式的逆波兰表达式写法为 ( ( 1 2 + ) ( 3 4 + ) * ) 。
 *
 * 说明：
 * 整数除法只保留整数部分。
 * 给定逆波兰表达式总是有效的。换句话说，表达式总会得出有效数值且不存在除数为 0 的情况。
 *
 * 示例
 * 输入: ["2", "1", "+", "3", "*"]
 * 输出: 9
 * 解释: 该算式转化为常见的中缀算术表达式为：((2 + 1) * 3) = 9
 *
 * 输入: ["4", "13", "5", "/", "+"]
 * 输出: 6
 * 解释: 该算式转化为常见的中缀算术表达式为：(4 + (13 / 5)) = 6
 *
 * 输入: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
 * 输出: 22
 * 解释:
 * 该算式转化为常见的中缀算术表达式为：
 * ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 */
public class Main {
    public static Map<String, Operation> operations = new HashMap<>();

    static {
        // 定义相关操作符对应的计算方法
        operations.put("+", new Operation("+", (num1, num2) -> num1 + num2));
        operations.put("-", new Operation("-", (num1, num2) -> num1 - num2));
        operations.put("*", new Operation("*", (num1, num2) -> num1 * num2));
        operations.put("/", new Operation("/", (num1, num2) -> num1 / num2));
    }

    public interface Cal {
        int cal(int num1, int num2);
    }

    public static class Operation {
        private final String opt;
        private final ReversePoland.Cal cal;

        public Operation(String opt, ReversePoland.Cal cal) {
            this.opt = opt;
            this.cal = cal;
        }

        public String getOpt() {
            return opt;
        }

        public ReversePoland.Cal getCal() {
            return cal;
        }
    }

    /**
     * 计算值
     */
    private int calculateNum(int num1, int num2, String opt) {
        Operation operation = operations.get(opt);
        if (operation == null) {
            throw new IllegalArgumentException("无效的运算符:" + opt);
        }
        if (operation.getOpt().equals("-") || operation.getOpt().equals("/")) {
            // 因为出栈后两数的位置颠倒，需交换两个数的位置
            num1 = num1 + num2;
            num2 = num1 - num2;
            num1 = num1 - num2;
        }
        return operation.getCal().cal(num1, num2);
    }

    public int evalRPN(String[] tokens) {
        // 模拟栈
        int[] numStacks = new int[tokens.length];
        // 数组当前元素下标
        int index = 0;
        for (String token : tokens) {
            Operation operation = operations.get(token);
            if (operation != null) {
                // 取出两个数计算
                int num1 = numStacks[index - 1];
                int num2 = numStacks[index - 2];
                int result = calculateNum(num1, num2, token);
                numStacks[index - 2] = result;
                index = index - 1;
                continue;
            }
            numStacks[index] = Integer.parseInt(token);
            index++;
        }
        // 第一个元素就是最后的结果
        return numStacks[0];
    }

    public static void main(String[] args) {
        ReversePoland reversePoland = new ReversePoland();

        String[] tokens = new String[]{"2", "1", "+", "3", "*"};
        int res = reversePoland.evalRPN(tokens);

        System.out.printf("%s\n", Arrays.toString(tokens));
        System.out.printf("%d\n", res);

        tokens = new String[]{"4", "13", "5", "/", "+"};
        res = reversePoland.evalRPN(tokens);

        System.out.printf("%s\n", Arrays.toString(tokens));
        System.out.printf("%d\n", res);

        tokens = new String[]{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"};
        res = reversePoland.evalRPN(tokens);

        System.out.printf("%s\n", Arrays.toString(tokens));
        System.out.printf("%d\n", res);
    }
}
