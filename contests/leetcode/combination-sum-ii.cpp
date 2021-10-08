// https://leetcode.com/problems/combination-sum-ii/
class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        vector<vector<int>> result;
        vector<int> current;
        set<int> forbid;
        recur(candidates, target, 0, 0, forbid, current, result);
        return result;
    }
    
    void recur(
        vector<int>& candidates, 
        int target,
        int pos,
        int sum,
        set<int>& forbid,
        vector<int>& current,
        vector<vector<int>>& result
    ) {
        if (sum == target) {
            result.push_back(current);
            return;
        }
        if (sum > target) return;
        if (pos == candidates.size()) return;
        
        int now = candidates[pos];
        
        if (forbid.find(now) == forbid.end()) {
            forbid.insert(now);
            recur(candidates, target, pos+1, sum, forbid, current, result);
            forbid.erase(forbid.find(now));

            current.push_back(now);
            recur(candidates, target, pos+1, sum + now, forbid, current, result);
            current.pop_back();
        } else {
            recur(candidates, target, pos+1, sum, forbid, current, result);
        }
    }
};