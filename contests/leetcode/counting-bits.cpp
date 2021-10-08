// https://leetcode.com/problems/counting-bits/
class Solution {
public:
    vector<int> countBits(int n) {
        if (n == 0) return {0};
        
        vector<int> ans(n+1, 0);
        ans[0] = 0;
        ans[1] = 1;
        int bit = 0;
        for (int i = 2; i <= n; ++i) {
            if (i >> (bit+1) != 0) bit++; 
            ans[i] = ans[i & ~(1 << bit)] + 1;
        }
        return ans;
    }
};