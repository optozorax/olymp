// https://leetcode.com/problems/subarray-product-less-than-k/
class Solution {
public:
    int numSubarrayProductLessThanK(vector<int>& nums, int k) {
        int l = 0;
        int64_t mul = 1;
        int64_t count = 0;
        for (int r = 0; r < nums.size(); ++r) {
            mul *= nums[r];
            while (l <= r && mul >= k) {
                mul /= nums[l];
                l++;
            }
            if (l <= r) {
                count += r - l + 1;
            }
        }
        return count;
    }
};