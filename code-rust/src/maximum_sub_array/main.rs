/// 最大子序和
/// 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

/// 示例
/// 输入: [-2,1,-3,4,-1,2,1,-5,4]
/// 输出: 6
/// 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。

pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 记录以每个元素结尾的最大值数组
        let mut max_array = vec![0; nums.len()];
        // 第一个元结尾的最大值
        max_array[0] = nums[0];
        // 初始化最大值
        let mut max_value = nums[0];
        for i in 1..nums.len() {
            max_array[i] = nums[i];
            // 前一个元素结尾的最大值
            if max_array[i - 1] + nums[i] > nums[i] {
                max_array[i] += max_array[i - 1];
            }
            // 判断最大值
            if max_array[i] > max_value {
                max_value = max_array[i];
            }
        }
        max_value
    }
}

#[test]
fn test_max_sub_array() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{:?}", nums);

    let value = Solution::max_sub_array(nums);
    println!("{}", value);
}
