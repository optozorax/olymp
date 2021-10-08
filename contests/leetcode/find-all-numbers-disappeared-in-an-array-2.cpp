// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        for (const auto& i1 : nums) {
            int i = abs(i1);
            if (nums[i-1] > 0) 
                nums[i-1] = -nums[i-1];
        }
        vector<int> ans;
        for (int i = 0; i < nums.size(); ++i) {
            if (nums[i] > 0) ans.push_back(i+1);
        }
        return ans;
    }
};