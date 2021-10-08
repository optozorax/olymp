// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        vector<bool> has(nums.size(), false);
        for (const auto& i : nums) has[i-1] = true;
        vector<int> ans;
        for (int i = 0; i < has.size(); ++i) {
            if (!has[i]) ans.push_back(i+1);
        }
        return ans;
    }
};