// https://leetcode.com/problems/find-all-duplicates-in-an-array/
class Solution {
public:
    vector<int> findDuplicates(vector<int>& nums) {
        vector<int> result;
        for (const auto& i : nums) {
            int i_abs = abs(i);
            if (nums[i_abs-1] < 0) result.push_back(i_abs);
            else nums[i_abs-1] = -nums[i_abs-1];
        }
        return result;
    }
};