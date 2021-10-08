// https://leetcode.com/problems/interval-list-intersections/
class Solution {
public:
    vector<vector<int>> intervalIntersection(
        vector<vector<int>>& a, 
        vector<vector<int>>& b
    ) {
        int ai = 0;
        int bi = 0;
        vector<vector<int>> result;
        while (ai != a.size() && bi != b.size()) {
            auto l = a[ai]; int* li = &ai;
            auto r = b[bi]; int* ri = &bi;
            if (l[0] > r[0]) { swap(l, r); swap(li, ri); }

            if (l[1] < r[0]) {
                (*li)++;
            } else {
                if (r[1] <= l[1]) {
                    result.push_back({r[0], r[1]});
                    (*ri)++;
                } else {
                    result.push_back({r[0], l[1]});
                    (*li)++;
                }
            }
        }
        return result;
    }
};