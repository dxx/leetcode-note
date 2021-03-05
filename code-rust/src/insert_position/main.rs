/// 搜索插入位置
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
///
/// 你可以假设数组中无重复元素。

/// 示例
/// 输入: [1,3,5,6], 5
/// 输出: 2

/// 输入: [1,3,5,6], 2
/// 输出: 1

/// 输入: [1,3,5,6], 7
/// 输出: 4

/// 输入: [1,3,5,6], 0
/// 输出: 0

pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        // 下标设置为数组长度，如果 target 大于数组中的所有元素，此时插入位置就是数组的长度
        let (mut start, mut end) = (0 as i32, (len - 1) as i32);
        let mut index = len;
        while start <= end {
            let mid = ((start + end) >> 1) as usize;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Greater => {
                    // 修改插入的下标
                    index = mid;
                    end = mid as i32 - 1;
                },
                std::cmp::Ordering::Less => {
                    start = mid as i32 + 1
                },
                _ => {
                    index = mid;
                    break;
                }
            }
        }
        index as i32
    }
}

#[test]
fn test_search_insert() {
    let nums: Vec<i32> = vec![1, 3, 5, 6];

    let target = 5;
    let index = Solution::search_insert(nums.clone(), target);

    println!("{:?}, {}", nums, target);
    println!("{}", index);

    let target = 2;
    let index = Solution::search_insert(nums.clone(), target);

    println!("{:?}, {}", nums, target);
    println!("{}", index);

    let target = 7;
    let index = Solution::search_insert(nums.clone(), target);

    println!("{:?}, {}", nums, target);
    println!("{}", index);

    let target = 0;
    let index = Solution::search_insert(nums.clone(), target);

    println!("{:?}, {}", nums, target);
    println!("{}", index);
}
