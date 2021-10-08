// https://leetcode.com/problems/sliding-window-maximum/
class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        deque<pair<int, int>> mq;
        for (int i = 0; i < k-1; ++i) {
            while (mq.size() > 0 && mq.back().first < nums[i]) mq.pop_back();
            mq.push_back({nums[i], i});
        }

        vector<int> result;
        result.reserve(nums.size()-k+1);
        for (int i = k-1; i < nums.size(); ++i) {
            while (mq.size() > 0 && mq.back().first < nums[i]) mq.pop_back();
            while (mq.size() > 0 && mq.front().second <= i-k) mq.pop_front();
            mq.push_back({nums[i], i});

            result.push_back(mq.front().first);
        }
        return result;
    }
};

/* 

resources of monotonic queue:
    * https://people.cs.uct.ac.za/~ksmith/articles/sliding_window_minimum.html
    * https://1e9.medium.com/monotonic-queue-notes-980a019d5793

 */