// https://leetcode.com/problems/subsets/
class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<vector<int>> result;
        vector<bool> take(nums.size(), false);
        recur(nums, take, 0, result);
        return result;
    }
    
    void recur(vector<int>& nums, vector<bool>& take, int index, vector<vector<int>>& result) {
        if (index == take.size()) {
            vector<int> to_push;
            for (int i = 0; i < take.size(); ++i) {
                if (take[i]) {
                    to_push.push_back(nums[i]);
                }
            }
            result.push_back(to_push);
        } else {
            take[index] = false;
            recur(nums, take, index+1, result);
            take[index] = true;
            recur(nums, take, index+1, result);
        }
    }
};