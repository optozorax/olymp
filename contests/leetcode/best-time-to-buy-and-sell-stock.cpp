// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int min = 100000;
        int best = 0;
        for (auto& i : prices) {
            if (i < min) min = i;
            if (i - min > best) best = i - min;
        }
        return best;
    }
};