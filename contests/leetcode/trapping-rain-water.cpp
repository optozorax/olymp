// https://leetcode.com/problems/trapping-rain-water/
class Solution {
public:
    int trap(vector<int>& h) {
        vector<int> nearest_right_greater(h.size(), 0);
        
        // create monotonic queue to find nearest right element that greater or equal to current by O(n)
        vector<pair<int, int>> mq;
        for (int i1 = 0; i1 < h.size(); ++i1) {
            int i = h.size() - 1 - i1;
            while (mq.size() > 0 && mq.back().first < h[i]) {
                mq.pop_back();
            }
            if (mq.empty()) {
                nearest_right_greater[i] = i;
            } else {
                nearest_right_greater[i] = mq.back().second;
            }
            mq.push_back(make_pair(h[i], i));
        }
        
        vector<pair<int, int>> suffix_max(h.size());
        for (int i1 = 0; i1 < h.size(); ++i1) {
            int i = h.size() - 1 - i1;
            if (i1 == 0) {
                suffix_max[i] = make_pair(h[i], i);
            } else {
                if (suffix_max[i+1].first > h[i]) {
                    suffix_max[i] = suffix_max[i+1];
                } else {
                    suffix_max[i] = make_pair(h[i], i);
                }
            }
        }
        
        vector<int> rs;
        int sum = 0;
        for (int i = 0; i < h.size(); ++i) {
            int r = nearest_right_greater[i];
            if (r == i && i != h.size() - 1) r = suffix_max[i+1].second;
            if (r == i) continue;
            while (rs.size() > 0 && rs.back() <= i)  rs.pop_back();
            if (rs.size() != 0) {
                sum -= h[i];
            } else {
                sum += (r-i-1) * min(h[i], h[r]);
            }
            rs.push_back(r);
        }
        
        return sum;
    }
};