// https://leetcode.com/problems/sliding-window-maximum/
class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        multiset<int> q;
        for (int i = 0; i < k; ++i) 
            q.insert(nums[i]);
        
        vector<int> result;
        for (int i = k; i < nums.size(); ++i) {
            result.push_back(*q.rbegin());
            q.erase(q.find(nums[i-k]));
            q.insert(nums[i]);
        }
        result.push_back(*q.rbegin());
        return result;
    }
};