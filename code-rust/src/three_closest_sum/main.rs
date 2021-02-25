/// 最接近的三数之和
/// 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。
/// 返回这三个数的和。假定每组输入只存在唯一答案。

/// 示例
/// 输入: nums = [-1,2,1,-4], target = 1
/// 输出: 2
/// 解释: 与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        // 防止后面 best_sum - target 溢出
        let mut best_sum = i32::MAX - target.abs();
        for i in 0..n {
            // 如果元素重复, 跳过
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;
            // 从剩下的数组中寻找所有两个数, 加上第一个数之和最接近 target 的目标和
            while left < right {
                let left_value = nums[left];
                let right_value = nums[right];
                let sum = nums[i] + left_value + right_value;

                // 相等直接返回
                if sum == target {
                    return target;
                }
                // 比较当前三数之和上一次和的差值
                if (sum - target).abs() < (best_sum - target).abs() {
                    best_sum = sum;
                }

                // 和小于目标值，从右边寻找下一个较大的值
                if sum < target {
                    left += 1;
                    // 如果元素相同指针后移
                    while left < right && nums[left] == left_value {
                        left += 1;
                    }
                    continue;
                }
                // 和大于目标值，从左边寻找下一个较小的值
                right -= 1;
                // 如果元素相同指针前移
                while left < right && nums[right] == right_value {
                    right -= 1;
                }
            }
        }
        best_sum
    }
}

#[test]
fn test_three_sum_closest() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    println!("nums = {:?}, target = {}", nums, target);

    let sum = Solution::three_sum_closest(nums, target);
    println!("{}", sum);
}
