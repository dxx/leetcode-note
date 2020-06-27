package partitionequalsubsetsum

// 分割等和子集
// 给定一个只包含正整数的非空数组。是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
// 注意:
//   每个数组中的元素不会超过 100
//   数组的大小不会超过 200

// 示例
// 输入: [1, 5, 11, 5]
// 输出: true
// 解释: 数组可以分割成 [1, 5, 5] 和 [11].

// 输入: [1, 2, 3, 5]
// 输出: false
// 解释: 数组不能分割成两个元素和相等的子集.

func canPartition(nums []int) bool {
    l := len(nums)
    if l < 2 {
        return false
    }

    var targetSum int
    var maxNum int
    for _, num := range nums {
        targetSum += num
        maxNum = max(maxNum, num)
    }
    // 和为奇数, 除以 2 后, 值不可能为整数, 返回 false
    if targetSum % 2 != 0 {
        return false
    }
    targetSum /= 2
    // 最大的元素大于所有元素和的一半, 返回 false
    if maxNum > targetSum {
        return false
    }

    // 保存前 i - 1 个元素中, 存在若干个元素和是否等于 j
    //dp := make([][]bool, l)
    //for i := 0; i < l; i++ {
    //    dp[i] = make([]bool, targetSum + 1)
    //    // 第一列, 值为 true
    //    dp[i][0] = true
    //}
    //// 第一行, 第一个元素的和 nums[0] 等于 nums[0]
    //dp[0][nums[0]] = true
    //
    //for i := 1; i < l; i++ {
    //    for j := 1; j <= targetSum; j++ {
    //        if nums[i] > j {
    //            // 当前元素的和大于 j, 当前元素不能选
    //            dp[i][j] = dp[i - 1][j]
    //        } else {
    //            // 当前元素可选, 前 i - 1 个元素中存在和为 j 或者前 i - 1 个元素中存在和为 j - 当前元素值
    //            dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i]]
    //        }
    //    }
    //}
    //return dp[l - 1][targetSum]

    // 保存前 i - 1 个元素中, 存在若干个元素和是否等于 j
    // 使用滑动数组优化
    dp := make([]bool, targetSum + 1)
    dp[0] = true
    // 第一个元素的和 nums[0] 等于 nums[0]
    dp[nums[0]] = true

    for i := 1; i < l; i++ {
        // 倒序遍历, 防止覆盖之前的记录
        for j := targetSum; j >= nums[i]; j-- {
            // 当前元素可选或不可选, 前 i - 1 个元素中存在和为 j 或者前 i - 1 个元素中存在和为 j - 当前元素值
            dp[j] = dp[j] || dp[j - nums[i]]
        }
    }
    return dp[targetSum]
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
