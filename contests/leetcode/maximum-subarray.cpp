// https://leetcode.com/problems/maximum-subarray/
class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int ans = nums[0];
        int prefix = 0;
        int min_prefix = 0;
        for (const auto& i : nums) {
            int current_sum = prefix + i - min_prefix;
            if (current_sum > ans) ans = current_sum;
            prefix += i;
            if (prefix < min_prefix) min_prefix = prefix;
        }
        return ans;
    }
};