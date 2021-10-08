// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        
        int s0 = 0;
        int s1 = -prices[0];
        int s2 = 0;
        
        for (int i = 0; i < n; ++i) {
            int new_s0 = max(s0, s2);
            int new_s1 = max(s1, s0 - prices[i]);
            int new_s2 = s1 + prices[i];
                
            s0 = new_s0;
            s1 = new_s1;
            s2 = new_s2;
        }

        return max(s0, s2);
    }
};

/* 

thanks https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/75928/

 */