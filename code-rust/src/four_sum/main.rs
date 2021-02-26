/// 四数之和
/// 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d
/// 使得 a + b + c + d 的值与 target 相等？找出所有满足条件且不重复的四元组。
/// 注意: 答案中不可以包含重复的四元组。

/// 示例
/// 输入: nums = [1, 0, -1, 0, -2, 2]，和 target = 0
/// 输出:
/// [
///   [-1,  0, 0, 1],
///   [-2, -1, 1, 2],
///   [-2,  0, 0, 2]
/// ]

pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut records = [i32::MIN; 4];
        let len = nums.len();
        if len == 0 {
            return res
        }
        Solution::n_sum(&mut nums, 0, len - 1, target, 4, &mut records, &mut res);
        res
    }

    /// 计算 n 个数之和
    fn n_sum(nums: &Vec<i32>, start: usize, end: usize, target: i32, n: u32, records: &mut [i32], res: &mut Vec<Vec<i32>>) {
        let record_len = records.len();
        if n <= 2 {
            // 计算两数之和
            let results: Vec<Vec<i32>> = Solution::two_sum(nums, start, end, target);
            // 存在两数之和等于目标值
            if results.len() > 0 {
                // 循环遍历结果, 将结果添加到最后的结果中
                for result in results {
                    records[record_len - 2] = result[0];
                    records[record_len - 1] = result[1];

                    let mut list = Vec::new();
                    for record in records.iter() {
                        list.push(*record);
                    }
                    res.push(list);
                }
            }
            return;
        }
        let mut i = start;
        while i <= end {
            // 如果元素重复, 跳过
            if nums[i] == records[record_len - n as usize] {
                i += 1;
                continue;
            }
            // 修改当前第 n 个数
            records[record_len - n as usize] = nums[i];
            if i + 1 < end {
                // 继续计算第 n - 1 个数之和
                Solution::n_sum(nums, i + 1, end, target - nums[i], n - 1, records, res);
            }
            // 表示下一次重新计算 n 数之和, 将向量元素重置
            if n as usize == record_len {
                for j in 1..record_len {
                    records[j] = i32::MIN;
                }
            }
            i += 1;
        }
    }

    /// 计算两数之和
    fn two_sum(nums: &Vec<i32>, start: usize, end: usize, target: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let (mut left, mut right) = (start, end);
        while left < right {
            let left_val = nums[left];
            let right_val = nums[right];
            let sum = left_val + right_val;
            // 如果相等，将元素放入集合
            if sum == target {
                let mut list = Vec::new();
                list.push(left_val);
                list.push(right_val);
                results.push(list);

                // 如果元素相同指针后移
                while left < right && nums[left] == left_val {
                    left += 1;
                }
                // 如果元素相同指针前移
                while left < right && nums[right] == right_val {
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
        results
    }
}

#[test]
fn test_four_sum() {
    let nums: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    println!("nums = {:?}, target = {}", nums.clone(), target);

    let sum = Solution::four_sum(nums, target);
    println!("{:?}", sum);
}
