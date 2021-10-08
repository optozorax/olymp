// https://leetcode.com/problems/combination-sum/
class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> result;
        vector<int> current;
        recur(candidates, target, 0, 0, current, result);
        return result;
    }
    
    void recur(
        vector<int>& candidates, 
        int target,
        int pos,
        int sum,
        vector<int>& current,
        vector<vector<int>>& result
    ) {
        if (sum == target) {
            result.push_back(current);
            return;
        }
        
        if (sum > target) return;
        
        for (int i = pos; i < candidates.size(); ++i) {
            current.push_back(candidates[i]);
            recur(candidates, target, i, sum + candidates[i], current, result);
            current.pop_back();
        }
    }
};