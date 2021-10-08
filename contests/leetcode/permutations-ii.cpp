// https://leetcode.com/problems/permutations-ii/
class Solution {
public:
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        vector<vector<int>> result;
        recur(nums, nums.size(), 0, result);
        return result;
    }
    
    void recur(vector<int>& current, int count, int index, vector<vector<int>>& result) {
        if (index == current.size()) {
            result.push_back(current);
        } else {
            set<int> used;
            for (int i = 0; i < count; ++i) {
                if (used.find(current[index+i]) == used.end()) {
                    swap(current[index], current[index+i]);
                    recur(current, count-1, index+1, result);
                    swap(current[index], current[index+i]);
                    
                    used.insert(current[index+i]);
                }
            }
        }
    }
};