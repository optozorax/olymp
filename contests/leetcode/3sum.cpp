// https://leetcode.com/problems/3sum/
class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        if (nums.size() < 3) return {};
        sort(nums.begin(), nums.end());
        vector<vector<int>> result;
        for (int i = 1; i < nums.size()-1; ++i) {
            if (nums[i] < 0 && i+1 != nums.size() - 1 && nums[i] == nums[i+1]) continue;
            if (nums[i] == 0 && ((nums[i-1] != 0 && nums[i+1] == 0) || (i != 1 && nums[i-2] == 0))) continue;
            if (nums[i] > 0 && i != 1 && nums[i] == nums[i-1]) continue;

            int lo = i-1;
            int hi = i+1;

            while (lo != -1 && hi != nums.size()) {
                int sum = nums[lo] + nums[i] + nums[hi];
                if (sum == 0) {
                    result.push_back({nums[lo], nums[i], nums[hi]});

                    int prev_lo = lo;
                    int prev_hi = hi;
                    while (lo != -1 && nums[lo] == nums[prev_lo]) lo--;
                    while (hi != nums.size() && nums[hi] == nums[prev_hi]) hi++;
                } else if (sum > 0) {
                    lo--;
                } else if (sum < 0) {
                    hi++;
                };
            }
        }
        return result;
    }
};