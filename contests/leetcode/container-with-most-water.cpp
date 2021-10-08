// https://leetcode.com/problems/container-with-most-water/
class Solution {
public:
    int maxArea(vector<int>& height) {
        vector<pair<int, int>> h;
        h.reserve(height.size());
        for (int i = 0; i < height.size(); ++i) {
            h.push_back(make_pair(-height[i], i));
        }
        sort(h.begin(), h.end());
        
        vector<int> most(height.size(), -1);
        int left = h[0].second;
        int right = h[0].second;
        for (auto& i : h) {
            if (abs(i.second - left) > abs(i.second - right)) {
                most[i.second] = left;
            } else {
                most[i.second] = right;
            }
            left = min(left, i.second);
            right = max(right, i.second);
        }
        
        int best = 0;
        for (int i = 0; i < height.size(); ++i) {
            best = max(best, abs(i - most[i]) * height[i]);
        }
        return best;
    }
};