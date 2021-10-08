// https://leetcode.com/problems/combination-sum-iv/
class Solution {
public:
    int combinationSum4(vector<int>& coins, int amount) {
        if (amount == 0) return 0;
        vector<uint64_t> dp(amount + 1, 0);
        for (const auto& i : coins) {
            if (i <= amount) {
                dp[i] = 1;
            }
        }
        for (int64_t i = 1; i < amount; ++i) {
            if (dp[i] != 0) {
                for (const auto& j : coins) {
                    if (i + j <= amount) {
                        dp[i + j] += dp[i];
                    }
                }
            }
        }
        return dp[amount];
    }
};