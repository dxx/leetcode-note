/// N 皇后
/// n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
/// 给定一个整数 n，返回 n 皇后不同的解决方案的数量。
/// 每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

/// 示例
/*
输入: 4
输出: 2
解释: 4 皇后问题存在如下两个不同的解法。
[
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]
*/

pub struct Queen {
    pub positions: Vec<i32>, // 记录每一次摆放的位置。下标表示第几个皇后，对应的值表示第几列
    pub total: i32, // 记录解决方案的数量
}

impl Queen {
    pub fn new(n: u32) -> Self {
        Queen {
            positions: vec![0; n as usize],
            total: 0,
        }
    }

    /// 放置皇后
    pub fn put_queen(&mut self, n: u32) {
        // 最后一个皇后摆放完成
        if n as usize == self.positions.len() {
            // 打印出摆放的位置
            // println!("{:?}", self.positions);

            self.total += 1;
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
}

pub struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut queen = Queen::new(n as u32);
        queen.put_queen(0);
        queen.total
    }
}

#[test]
fn test_total_n_queens() {
    let n = 4;
    let total = Solution::total_n_queens(n);
    println!("{}", n);
    println!("{:?}", total);
}
