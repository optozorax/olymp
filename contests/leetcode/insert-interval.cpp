// https://leetcode.com/problems/insert-interval/
class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        if (intervals.size() == 0) return { newInterval };
        
        vector<vector<int>> result;
        auto current = intervals[0];
        if (newInterval[1] < current[0]) {
            result.push_back(newInterval);
        }
        for (auto& i : intervals) {
            if (intersects(current, i)) {
                current = merge(current, i);
            } else {
                result.push_back(current);
                if (current[1] < newInterval[0] && newInterval[1] < i[0]) {
                    result.push_back(newInterval);
                }
                current = i;
            }
            
            if (intersects(current, newInterval)) {
                current = merge(current, newInterval);
            }
        }
        result.push_back(current);
        if (current[1] < newInterval[0]) {
            result.push_back(newInterval);
        }
        return result;
    }
    
    bool intersects(vector<int>& a, vector<int>& b) {
        return (b[0] <= a[0] && a[0] <= b[1]) || (a[0] <= b[0] && b[0] <= a[1]);
    }
    
    vector<int> merge(vector<int>& a, vector<int>& b) {
        return {min(a[0], b[0]), max(a[1], b[1])};
    }
};