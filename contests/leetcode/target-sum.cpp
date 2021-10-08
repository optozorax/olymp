// https://leetcode.com/problems/target-sum/
class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int target) {
        map<int, int> dp;
        dp[nums[0]] += 1;
        dp[-nums[0]] += 1;
        
        map<int, int> dp2;
        for (int i = 1; i < nums.size(); ++i) {
            for (auto& [k, v] : dp) {
                dp2[k+nums[i]] += v;
                dp2[k-nums[i]] += v;
            }
            swap(dp, dp2);
            dp2.clear();
        }
        
        return dp[target];
    }
};