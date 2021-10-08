// https://leetcode.com/problems/single-number/
class Solution {
public:
    int singleNumber(vector<int>& nums) {
        int result = 0;
        for (const auto& i : nums) result ^= i;
        return result;
    }
};