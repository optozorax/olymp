// https://leetcode.com/problems/subarray-sum-equals-k/
class Solution {
public:
    int subarraySum(vector<int>& nums, int k) {
        unordered_map<int, int> sums;
        int sum = 0;
        int count = 0;
        for (int i = 0; i < nums.size(); ++i) {
            sums[sum]++;
            sum += nums[i];
            if (sums[sum - k] != 0) {
                count += sums[sum - k];
            }
        }
        return count;
    }
};