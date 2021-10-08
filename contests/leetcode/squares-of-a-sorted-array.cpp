// https://leetcode.com/problems/squares-of-a-sorted-array/
class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        vector<int> result;
        result.reserve(nums.size());

        int pos = 0;
        while (pos != nums.size() && nums[pos] < 0) pos++;

        int l = pos-1;
        int r = pos;
        while (!(l == -1 && r == nums.size())) {
            if (l == -1) {
                result.push_back(nums[r] * nums[r]);
                r++;
            } else if (r == nums.size()) {
                result.push_back(nums[l] * nums[l]);
                l--;
            } else {
                if (-nums[l] > nums[r]) {
                    result.push_back(nums[r] * nums[r]);
                    r++;
                } else {
                    result.push_back(nums[l] * nums[l]);
                    l--;
                }
            }
        }

        return result;
    }
};