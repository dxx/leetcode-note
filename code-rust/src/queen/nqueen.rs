/// N 皇后
/// n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
/// 给定一个整数 n，返回所有不同的 n 皇后问题的解决方案。
/// 每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

/// 示例
/*
输入: 4
输出: [
[".Q..",  // 解法 1
"...Q",
"Q...",
"..Q."],

["..Q.",  // 解法 2
"Q...",
"...Q",
".Q.."]
]
解释: 4 皇后问题存在两个不同的解法。
*/

pub struct Queen {
    pub positions: Vec<i32>, // 记录每一次摆放的位置。下标表示第几个皇后，对应的值表示第几列
    pub outputs: Vec<Vec<String>>, // 保存输出的结果
}

impl Queen {
    pub fn new(n: u32) -> Self {
        Queen {
            positions: vec![0; n as usize],
            outputs: vec![],
        }
    }

    /// 放置皇后
    pub fn put_queen(&mut self, n: u32) {
        // 最后一个皇后摆放完成
        if n as usize == self.positions.len() {
            // 打印出摆放的位置
            // println!("{:?}", self.positions);

            // 根据摆放的位置创建数组
            self.add_solution(self.positions.clone());
            return;
        }
        // 有几个位置就有几个皇后
        for i in 0..self.positions.len() {
            // i = 0 时，假设当前皇后可以放在第一列
            // 如果不能放，将进行下一次循环，当前皇后放在下一个位置
            self.positions[n as usize] = i as i32;
            if self.is_not_conflict(n as usize) {
                // 继续摆放下一皇后
                self.put_queen(n + 1);
            }
        }
    }

    // 判断当前皇后是否和已经摆放过的皇后冲突
    pub fn is_not_conflict(&self, n: usize) -> bool {
        for i in 0..n {
            // positions[n] == positions[i] 表示在同一列
            // abs((n - i)) == abs((positions[n] - positions[i]) 表示同一斜线
            if self.positions[n] == self.positions[i] ||
                (n as i32 - i as i32).abs() == (self.positions[n] - self.positions[i]).abs() {
                return false;
            }
        }
        true
    }

    fn add_solution(&mut self, positions: Vec<i32>) {
        let length = positions.len();
        let mut output: Vec<String> = vec!["".to_string(); length];

        for i in 0..length {
            let mut string = String::new();
            for _j in 0..positions[i] {
                string.push('.');
            }
            string.push('Q');
            for _j in positions[i] + 1..length as i32 {
                string.push('.');
            }
            output[i] = string;
        }

        self.outputs.push(output);
    }
}

pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut queen = Queen::new(n as u32);
        queen.put_queen(0);
        queen.outputs
    }
}

#[test]
fn test_solve_n_queens() {
    let n = 4;
    let outputs = Solution::solve_n_queens(n);
    println!("{}", n);
    println!("{:?}", outputs);
}
