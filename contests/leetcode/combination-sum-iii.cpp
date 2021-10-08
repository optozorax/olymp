// https://leetcode.com/problems/combination-sum-iii/
class Solution {
public:
    vector<vector<int>> combinationSum3(int k, int target) {
        vector<vector<int>> result;
        vector<int> current;
        recur(target, k, 1, 0, current, result);
        return result;
    }
    
    void recur(
        int target,
        int k,
        int pos,
        int sum,
        vector<int>& current,
        vector<vector<int>>& result
    ) {
        if (sum == target && current.size() == k) {
            result.push_back(current);
            return;
        }
        
        if (sum > target) return;
        
        if (current.size() == k) return;
        
        for (int i = pos; i < 10; ++i) {
            current.push_back(i);
            recur(target, k, i + 1, sum + i, current, result);
            current.pop_back();
        }
    }
};