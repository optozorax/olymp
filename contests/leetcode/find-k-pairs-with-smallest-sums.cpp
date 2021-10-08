// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
class Solution {
public:
    vector<vector<int>> kSmallestPairs(vector<int>& a, vector<int>& b, int k) {
        multimap<int, pair<int, int>> m;
        m.insert({a[0] + b[0],  {0, 0}});
        
        set<pair<int, int>> result;
        while (result.size() != k && m.size() != 0) {
            auto smallest = (*m.begin());
            m.erase(m.begin());
            
            int ai = smallest.second.first;
            int bi = smallest.second.second;
            if (result.insert(smallest.second).second) {
                if (ai + 1 < a.size()) m.insert({a[ai + 1] + b[bi], {ai + 1, bi}});
                if (bi + 1 < b.size()) m.insert({a[ai] + b[bi + 1], {ai, bi + 1}});
            }
        }
        
        vector<vector<int>> result2;
        for (auto& i : result) {
            result2.push_back({a[i.first], b[i.second]});
        }
        return result2;
    }
};