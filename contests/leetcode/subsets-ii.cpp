// https://leetcode.com/problems/subsets-ii/
class Solution {
public:
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        vector<vector<int>> result;
        vector<bool> take(nums.size(), false);
        set<int> forbid;
        recur(nums, take, 0, forbid, result);
        return result;
    }
    
    void recur(vector<int>& nums, vector<bool>& take, int index, set<int>& forbid, vector<vector<int>>& result) {
        if (index == take.size()) {
            vector<int> to_push;
            for (int i = 0; i < take.size(); ++i) {
                if (take[i]) {
                    to_push.push_back(nums[i]);
                }
            }
            result.push_back(to_push);
        } else {
            if (forbid.find(nums[index]) == forbid.end()) {
                take[index] = false;
                forbid.insert(nums[index]);
                recur(nums, take, index+1, forbid, result);
                forbid.erase(forbid.find(nums[index]));
                
                take[index] = true;
                recur(nums, take, index+1, forbid, result);
            } else {
                take[index] = false;
                recur(nums, take, index+1, forbid, result);
            }
        }
    }
};