/// 三数之和
/// 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
/// 注意: 答案中不可以包含重复的三元组。

/// 示例
/// 输入: nums = [-1, 0, 1, 2, -1, -4]
/// 输出:
/// [
///   [-1, 0, 1],
///   [-1, -1, 2]
/// ]

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut results: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();
        for i in 0..n {
            // 如果元素重复, 跳过
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let target = -nums[i];
            let mut left = i + 1;
            let mut right = n - 1;
            // 从剩下的数组中寻找所有两个数之和等于 target 的两个元素
            while left < right {
                let left_value = nums[left];
                let right_value = nums[right];
                let sum = left_value + right_value;
                // 寻找到两个数, 放入结果集中
                if sum == target {
                    let mut list =  Vec::new();
                    list.push(nums[i]);
                    list.push(nums[left]);
                    list.push(nums[right]);
                    results.push(list);

                    // 如果元素相同指针后移
                    while left < right && nums[left] == left_value {
                        left += 1;
                    }
                    // 如果元素相同指针前移
                    while left < right && nums[right] == right_value {
                        right -= 1;
                    }
                    continue;
                }
                // 和小于目标值，从右边寻找下一个较大的值
                if sum < target {
                    left += 1;
                    continue;
                }
                // 和大于目标值，从左边寻找下一个较小的值
                right -= 1;
            }
        }
        results
    }
}

#[test]
fn test_three_sum() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    println!("nums = {:?}", nums);

    let results = Solution::three_sum(nums);
    println!("{:?}", results);
}
