// https://leetcode.com/problems/house-robber-ii/
class Solution {
public:
    int rob(vector<int>& nums) {
        if (nums.size() == 0) return 0;
        if (nums.size() == 1) return nums[0];
        if (nums.size() == 2) return max(nums[0], nums[1]);
        
        auto with_first = rob1(nums, true);
        nums.erase(nums.begin());
        auto without_first = rob1(nums, false);
        
        return max(with_first, without_first);
    }
    
    int rob1(vector<int>& nums, bool with_first) {
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
        if (with_first) {
            dp.pop_back();
        }
        return *max_element(dp.begin(), dp.end());
    }
};