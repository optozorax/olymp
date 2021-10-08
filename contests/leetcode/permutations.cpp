// https://leetcode.com/problems/permutations/
class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> result;
        recur(nums, nums.size(), 0, result);
        return result;
    }
    
    void recur(vector<int>& current, int count, int index, vector<vector<int>>& result) {
        if (index == current.size()) {
            result.push_back(current);
        } else {
            for (int i = 0; i < count; ++i) {
                swap(current[index], current[index+i]);
                recur(current, count-1, index+1, result);
                swap(current[index], current[index+i]);
            }
        }
    }
};