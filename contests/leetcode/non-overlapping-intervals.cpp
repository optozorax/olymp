// https://leetcode.com/problems/non-overlapping-intervals/
class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end(), [] (const auto& l, const auto& r) {
            return l[0] < r[0];
        });
        int count = 0;
        auto previous = intervals[0];
        for (int i = 1; i < intervals.size(); ++i) {
            if (previous[1] > intervals[i][0]) {
                count++;
                if (previous[1] > intervals[i][1]) {
                    previous = intervals[i];
                }
            } else {
                previous = intervals[i];
            }
        }
        return count;
    }
};