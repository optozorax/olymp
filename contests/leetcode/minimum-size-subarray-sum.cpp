// https://leetcode.com/problems/minimum-size-subarray-sum/
class Solution {
public:
    int minSubArrayLen(int target, vector<int>& nums) {
        vector<int> prefix(nums.size() + 1);
        prefix[0] = 0;
        for (int i = 1; i <= nums.size(); ++i) {
            prefix[i] = nums[i-1] + prefix[i-1];
        }
        if (prefix.back() < target) return 0;
        
        int l = 0; 
        int best = nums.size();
        for (int r = 0; r < nums.size(); ++r) {
            while (prefix[r+1] - prefix[l] >= target && l < r+1) {
                best = min(best, r+1-l);
                l++;
            }
        }
        return best;
    }
};