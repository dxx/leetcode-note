use std::cmp::min;

/// 寻找两个正序数组的中位数
/// 给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。
/// 请你找出这两个正序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
/// 你可以假设 nums1 和 nums2 不会同时为空。

/// 示例1
/// nums1 = [1, 3]
/// nums2 = [2]
/// 则中位数是 2.0

/// 示例2
/// nums1 = [1, 2]
/// nums2 = [3, 4]
/// 则中位数是 (2 + 3)/2 = 2.5

pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 归并查找，时间复杂度 O(m+n)
        // Solution::merge_search(nums1, nums2)

        // 二分查找，时间复杂度 O(log(m+n))
        Solution::binary_search(nums1, nums2)
    }

    fn binary_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (len1, len2) = (nums1.len(), nums2.len());
        let total_len = len1 + len2;
        if total_len % 2 != 0 {
            // 奇数
            Solution::get_kth_element(nums1, nums2, total_len / 2 + 1) as f64
        } else {
            // 偶数
            let median1 =
                Solution::get_kth_element(nums1.clone(), nums2.clone(), total_len / 2);
            let median2 =
                Solution::get_kth_element(nums1.clone(), nums2.clone(), total_len / 2 + 1);
            (median1 + median2) as f64 / 2.0
        }
    }

    /// 获取两个数组中第 k 个元素
    fn get_kth_element(nums1: Vec<i32>, nums2: Vec<i32>, k: usize) -> i32 {
        let (len1, len2) = (nums1.len(), nums2.len());
        let (mut index1, mut index2) = (0, 0);
        let mut mut_k = k;
        loop {
            // 数组中的所有元素都被排除，返回另一个数组中第 k 小的元素
            if index1 == len1 {
                return nums2[index2 + mut_k - 1];
            }
            if index2 == len2 {
                return nums1[index1 + mut_k - 1];
            }
            // 如果 k=1，返回两个数组首个元素的最小值
            if mut_k == 1 {
                return min(nums1[index1], nums2[index2]);
            }

            let half = mut_k / 2;
            let new_index1 = min(index1 + half, len1) - 1;
            let new_index2 = min(index2 + half, len2) - 1;
            if nums1[new_index1] <= nums2[new_index2] {
                // 排除了元素后，k 减少排除元素的个数
                mut_k -= new_index1 - index1 + 1;
                // 记录排除元素后数组的起始下标
                index1 = new_index1 + 1;
            } else {
                mut_k -= new_index2 - index2 + 1;
                index2 = new_index2 + 1;
            }
        }
    }

    #[allow(dead_code)]
    fn merge_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut index1, mut index2, mut merge_num_index) = (0, 0, 0);
        let (len1, len2) = (nums1.len(), nums2.len());

        // 创建一个向量用来保存合并的元素，只需要保存到中间的元素即可
        let mut merge_nums = vec![0; (len1 + len2) / 2 + 1];
        while merge_num_index < merge_nums.len() {
            let mut value1 = 1 << 31 - 1;
            let mut value2 = 1 << 31 - 1;
            if index1 < len1 {
                value1 = nums1[index1];
            }
            if index2 < len2 {
                value2 = nums2[index2];
            }
            if value1 <= value2 {
                merge_nums[merge_num_index] = value1;
                index1 += 1;
            } else {
                merge_nums[merge_num_index] = value2;
                index2 += 2;
            }
            merge_num_index += 1;
        }

        if (len1 + len2) & 2 != 0 {
            // 奇数取最后一个元素
            merge_nums[merge_nums.len() - 1] as f64
        } else {
            // 偶数取最后两个元素的平均值
            (merge_nums[merge_nums.len() - 2] + merge_nums[merge_nums.len() - 1]) as f64 / 2.0
        }
    }
}

#[test]
fn test_find_median_sorted_arrays() {
    let nums1: Vec<i32> = vec![1, 3];
    let nums2: Vec<i32> = vec![2];
    let median = Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());

    println!("{:?}", nums1);
    println!("{:?}", nums2);
    println!("中位数是 {}", median);

    let nums1: Vec<i32> = vec![1, 2];
    let nums2: Vec<i32> = vec![3, 4];
    let median = Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());

    println!("{:?}", nums1);
    println!("{:?}", nums2);
    println!("中位数是 {}", median);
}
