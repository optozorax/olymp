// https://leetcode.com/problems/minimum-height-trees/
class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        vector<vector<int>> adj(n);
        for (auto& i : edges) {
            adj[i[0]].push_back(i[1]);
            adj[i[1]].push_back(i[0]);
        }
        
        unordered_map<int64_t, int> depth;
        for (auto& i : edges) {
            recur(adj, depth, i[0], i[1]);
            recur(adj, depth, i[1], i[0]);
        }
        
        vector<int> count(n);
        for (int i = 0; i < n; ++i) {
            for (auto& j : adj[i]) {
                count[i] = max(count[i], depth[make_pair(i, j)]);
            }
        }
        
        int best = *min_element(count.begin(), count.end());
        
        vector<int> result;
        for (int i = 0; i < n; ++i) {
            if (count[i] == best) 
                result.push_back(i);
        }
        return result;
    }
    
    void recur(
        const vector<vector<int>>& adj,
        unordered_map<int64_t, int>& depth,
        int previous,
        int current
    ) {
        if (depth[make_pair(previous, current)] != 0) return;
        int best = 0;
        for (auto& i : adj[current]) {
            if (i != previous) {
                recur(adj, depth, current, i);
                best = max(best, depth[make_pair(current, i)]);
            }
        }
        depth[make_pair(previous, current)] = best+1;
    }
    
    int64_t make_pair(int a, int b) {
        return (int64_t(a) << 32) | int64_t(b);
    }
};