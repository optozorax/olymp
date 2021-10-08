// https://leetcode.com/problems/k-closest-points-to-origin/
class Solution {
public:
    vector<vector<int>> kClosest(vector<vector<int>>& points, int k) {
        multimap<int, vector<int>> m;
        for (auto& i : points) {
            int dist = i[0]*i[0] + i[1]*i[1];
            m.insert({-dist, i});
            if (m.size() > k) {
                m.erase(m.begin());
            }
        }
        vector<vector<int>> result;
        for (auto& i : m) {
            result.push_back(i.second);
        }
        return result;
    }
};