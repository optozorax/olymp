// https://leetcode.com/problems/coin-change/
class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        if (amount == 0) return 0;
        vector<int> dp(amount + 1, 0);
        for (const auto& i : coins) {
            if (i <= amount) {
                dp[i] = 1;
            }
        }
        for (int64_t i = 1; i < amount; ++i) {
            if (dp[i] != 0) {
                for (const auto& j : coins) {
                    if (i + j <= amount) {
                        if (dp[i + j] == 0) {
                            dp[i + j] = dp[i] + 1;
                        } else {
                            dp[i + j] = min(dp[i + j], dp[i] + 1);
                        }
                    }
                }
            }
        }
        if (dp[amount] == 0) return -1;
        return dp[amount];
    }
};