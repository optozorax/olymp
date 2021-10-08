// https://leetcode.com/problems/missing-number/
class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int sum = 0;
        for (int i = 0; i < nums.size(); ++i) {
            sum += nums[i];
        }
        int should_be = (nums.size()) * (nums.size() + 1) / 2;
        return should_be - sum;
    }
};