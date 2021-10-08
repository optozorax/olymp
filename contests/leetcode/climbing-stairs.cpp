// https://leetcode.com/problems/climbing-stairs/
class Solution {
public:
    int climbStairs(int n) {
        int result = 1;
        int prev1 = 1;
        int prev2 = 1;
        for (int i = 1; i < n; ++i) {
            result = prev1 + prev2;
            prev2 = prev1;
            prev1 = result;
        }
        return result;
    }
};