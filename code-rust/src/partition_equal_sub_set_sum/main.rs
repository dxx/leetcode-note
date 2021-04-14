/// 分割等和子集
/// 给定一个只包含正整数的非空数组。是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
/// 注意:
///   每个数组中的元素不会超过 100
///   数组的大小不会超过 200

/// 示例
/// 输入: [1, 5, 11, 5]
/// 输出: true
/// 解释: 数组可以分割成 [1, 5, 5] 和 [11].

/// 输入: [1, 2, 3, 5]
/// 输出: false
/// 解释: 数组不能分割成两个元素和相等的子集。

pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let l = nums.len();
        if l < 2 {
            return false;
        }
        let mut target_sum = 0;
        let mut max_num = 0;
        for num in nums.iter() {
            target_sum += *num;
            max_num = max_num.max(*num);
        }
        // 和为奇数, 除以 2 后, 值不可能为整数, 返回 false
        if target_sum % 2 != 0 {
            return false;
        }
        target_sum /= 2;
        // 最大的元素大于所有元素和的一半, 返回 false
        if max_num > target_sum {
            return false;
        }

        // 保存前 i - 1 个元素中, 存在若干个元素和是否等于 j
        // 使用滑动数组优化
        let mut dp = vec![false; target_sum as usize + 1];
        dp[0] = true;
        // 第一个元素的和 nums[0] 等于 nums[0]
        dp[nums[0] as usize] = true;

        for i in 1..l {
            // 倒序遍历, 防止覆盖之前的记录
            for j in (nums[i] as usize..=target_sum as usize).rev() {
                // 当前元素可选或不可选, 前 i - 1 个元素中存在和为 j 或者前 i - 1 个元素中存在和为 j - 当前元素值
                dp[j] = dp[j] || dp[j - nums[i] as usize];
            }
        }
        dp[target_sum as usize]
    }
}

#[test]
fn test_can_partition() {
    let nums = vec![1, 5, 11, 5];
    println!("{:?}", nums);

    let result = Solution::can_partition(nums.clone());
    println!("{}", result);

    let nums = vec![1, 2, 3, 5];
    println!("{:?}", nums);

    let result = Solution::can_partition(nums.clone());
    println!("{}", result);
}
