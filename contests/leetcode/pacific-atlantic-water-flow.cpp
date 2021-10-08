// https://leetcode.com/problems/pacific-atlantic-water-flow/
class Solution {
public:
    vector<vector<int>> pacificAtlantic(vector<vector<int>>& h) {
        vector<vector<bool>> pacific(h.size(), vector<bool>(h[0].size(), false));
        for (int i = 0; i < h.size(); ++i) visit(pacific, h, i, 0);
        for (int i = 0; i < h[0].size(); ++i) visit(pacific, h, 0, i);
        
        vector<vector<bool>> atlantic(h.size(), vector<bool>(h[0].size(), false));
        for (int i = 0; i < h.size(); ++i) visit(atlantic, h, i, h[0].size()-1);
        for (int i = 0; i < h[0].size(); ++i) visit(atlantic, h, h.size()-1, i);
        
        vector<vector<int>> result;
        for (int i = 0; i < h.size(); ++i) {
            for (int j = 0; j < h[0].size(); ++j) {
                if (pacific[i][j] && atlantic[i][j]) {
                    result.push_back({i, j});
                } 
            }
        }
        return result;
    }
    
    void visit(vector<vector<bool>>& ans, vector<vector<int>>& h, int x, int y) {
        if (ans[x][y]) return;
        
        ans[x][y] = true;
        int current = h[x][y];
        if (x > 0 && h[x-1][y] >= current) visit(ans, h, x-1, y);
        if (y > 0 && h[x][y-1] >= current) visit(ans, h, x, y-1);
        if (x+1 < h.size() && h[x+1][y] >= current) visit(ans, h, x+1, y);
        if (y+1 < h[x].size() && h[x][y+1] >= current) visit(ans, h, x, y+1);
    }
};