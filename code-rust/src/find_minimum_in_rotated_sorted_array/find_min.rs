/// 寻找旋转排序数组中的最小值
/// 假设按照升序排序的数组在预先未知的某个点上进行了旋转。 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2],
/// 请找出其中最小的元素。

/// 你可以假设数组中不存在重复元素。

/// 示例
/// 输入: [3,4,5,1,2]
/// 输出: 1

/// 输入: [4,5,6,7,0,1,2]
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
            } else {
                // 最小值比中间元素小，最小值在中间元素的右边
                start = mid + 1;
            }
        }
        nums[end]
    }
}

#[test]
fn test_find_min() {
    let nums: Vec<i32> = vec![3, 4, 5, 1, 2];
    let min = Solution::find_min(nums.clone());

    println!("{:?}", nums);
    println!("{}", min);

    let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
    let min = Solution::find_min(nums.clone());

    println!("{:?}", nums);
    println!("{}", min);
}
