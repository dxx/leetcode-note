use std::collections::HashMap;

/// 两数之和
/// 给定一个整数数组 `nums` 和一个目标值 `target`，请你在该数组中找出和为目标值的那**两个**整数，并返回他们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

/// 示例
/// 给定 nums = [2, 7, 11, 15], target = 9
/// 因为 nums[0] + nums[1] = 2 + 7 = 9
/// 所以返回 [0, 1]

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexes = Vec::new();

        // 时间复杂度 O(n^2), 空间复杂度 O(1)
        /*for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    indexes.push(i as i32);
                    indexes.push(j as i32);
                    break;
                }
            }
        }*/

        let mut num_map = HashMap::new();
        // 时间复杂度 O(n), 空间复杂度 O(n)
        for i in 0..nums.len() {
            // 计算补数
            let n = target - nums[i];
            // 判断补数是否存在
            match num_map.get(&n) {
                Some(index) => {
                    // 补数存在，将补数的小标和当前数的下标放入数组中
                    indexes.push(*index as i32);
                    indexes.push(i as i32);
                    break;
                }
                None => {
                    // 将当前元素放入 map 中，方便后续判断
                    num_map.insert(nums[i], i);
                }
            };
        }
        indexes
    }
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
