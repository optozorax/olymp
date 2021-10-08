// https://leetcode.com/problems/merge-intervals/
class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end(), [](const auto& l, const auto& r){
            return l[0] < r[0];
        });
        
        int start = intervals[0][0];
        int end = intervals[0][1];
        vector<vector<int>> result;
        for (const auto& i : intervals) {
            if (start <= i[0] && i[0] <= end) {
                end = max(end, i[1]);
            } else {
                result.push_back({start, end});
                start = i[0];
                end = i[1];
            }
        }
        result.push_back({start, end});
        return result;
    }
};