/// 寻找旋转排序数组中的最小值 II
/// 假设按照升序排序的数组在预先未知的某个点上进行了旋转。例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2],
/// 请找出其中最小的元素。

/// 注意数组中可能存在重复的元素。

/// 示例
/// 输入: [1,3,5]
/// 输出: 1

/// 输入: [2,2,2,0,1]
/// 输出: 0

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);
        // 二分查找
        while start < end {
            let mid = (start + end) >> 1;
            // 最小值比中间元素大，说明最小值在中间元素的左边
            if nums[end].gt(&nums[mid]) {
                end = mid;
            } else if nums[end].lt(&nums[mid]) {
                // 最小值比中间元素小，最小值在中间元素的右边
                start = mid + 1;
            } else {
                // 相等，end 前移一位
                end -= 1;
            }
        }
        nums[end]
    }
}

#[test]
fn test_find_min() {
    let nums: Vec<i32> = vec![1, 3, 5];
    let min = Solution::find_min(nums.clone());

    println!("{:?}", nums);
    println!("{}", min);

    let nums: Vec<i32> = vec![2, 2, 2, 0, 1];
    let min = Solution::find_min(nums.clone());

    println!("{:?}", nums);
    println!("{}", min);
}
