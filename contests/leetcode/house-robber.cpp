// https://leetcode.com/problems/house-robber/
class Solution {
public:
    int rob(vector<int>& nums) {
        vector<int> dp(nums.size(), 0);
        for (int i = 0; i < nums.size(); ++i) {
            if (i > 2) {
                dp[i] = max(dp[i-2], dp[i-3]) + nums[i];
            } else if (i > 1) {
                dp[i] = dp[i-2] + nums[i];
            } else {
                dp[i] = nums[i];
            }
        }
        return *max_element(dp.begin(), dp.end());
    }
};