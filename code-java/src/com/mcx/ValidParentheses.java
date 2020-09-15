package com.mcx;

import java.util.Stack;

/**
 * Created by mcx on 2020-09-15.
 *
 * 有效的括号
 * 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
 * 有效字符串需满足：
 *    左括号必须用相同类型的右括号闭合。
 *    左括号必须以正确的顺序闭合。
 * 注意空字符串可被认为是有效字符串。
 *
 * 示例
 * 输入: "()"
 * 输出: true
 *
 * 输入: "()[]{}"
 * 输出: true
 *
 * 输入: "(]"
 * 输出: false
 *
 * 输入: "([)]"
 * 输出: false
 *
 * 输入: "{[]}"
 * 输出: true
 */
public class ValidParentheses {

    public static boolean isValid(String s) {
        Stack<Character> stack = new Stack<>();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            // 将（ { [ 入栈
            if (c == '(' || c == '{' || c == '[') {
                stack.push(c);
                continue;
            }
            // 如果栈为空,表示 ( { [ 中没有一个符号入栈
            if (stack.isEmpty()) {
                return false;
            }
            char ch = stack.pop();
            switch (c) {
                case ')':
                    // 判断是否和栈顶元素配对
                    if (ch != '(') {
                        return false;
                    }
                    break;
                case '}':
                    if (ch != '{') {
                        return false;
                    }
                    break;
                case ']':
                    if (ch != '[') {
                        return false;
                    }
                    break;
            }
        }
        // 所有的括号能配对, 栈就会为空
        return stack.isEmpty();
    }

    public static void main(String[] args) {
        String s = "()";
        System.out.println(s);

        boolean r = isValid(s);
        System.out.println(r);

        s = "()[]{}";
        System.out.println(s);

        r = isValid(s);
        System.out.println(r);

        s = "(]";
        System.out.println(s);

        r = isValid(s);
        System.out.println(r);

        s = "([)]";
        System.out.println(s);

        r = isValid(s);
        System.out.println(r);

        s = "{[]}";
        System.out.println(s);

        r = isValid(s);
        System.out.println(r);
    }
}
