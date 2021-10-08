// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
class Solution {
public:
    int findMinArrowShots(vector<vector<int>>& points) {
        vector<bool> bursted(points.size(), false);
        
        priority_queue<pair<int, int>> starts;
        priority_queue<pair<int, int>> ends;
        
        for (int i = 0; i < points.size(); ++i) {
            starts.push(make_pair(points[i][0], i));
            ends.push(make_pair(points[i][1], i));
        }
        
        int count = 0;
        while (starts.size() != 0) {
            auto top = starts.top(); starts.pop();
            
            if (bursted[top.second]) continue;
            bursted[top.second] = true;
            
            count++;
            
            while (ends.size() != 0 && ends.top().first >= top.first) {
                auto top_end = ends.top(); ends.pop();
                bursted[top_end.second] = true;
            }
        }
        return count;
    }
};