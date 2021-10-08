// https://leetcode.com/problems/minimum-height-trees/
class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        if (n == 1) return {0};
        if (n == 2) return {0, 1};
        
        vector<vector<int>> adj(n);
        vector<int> counts(n, 0);
        for (auto& i : edges) {
            adj[i[0]].push_back(i[1]);
            adj[i[1]].push_back(i[0]);
            counts[i[0]]++;
            counts[i[1]]++;
        }
        
        vector<bool> visited(n, false);
        
        vector<int> to_visit;
        for (int i = 0; i < n; ++i) {
            if (counts[i] == 1) {
                to_visit.push_back(i);
                visited[i] = true;
            }
        }
        
        int count = n;
        vector<int> to_visit2;
        while (count > 2) {
            to_visit2.clear();
            for (auto& i : to_visit) {
                count--;
                for (auto& j : adj[i]) {
                    counts[j]--;
                    if (!visited[j] && counts[j] == 1) {
                        to_visit2.push_back(j);
                        visited[j] = true;
                    }
                }
            }
            swap(to_visit2, to_visit);
        }
        
        return to_visit;
    }
};