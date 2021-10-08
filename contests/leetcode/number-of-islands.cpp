// https://leetcode.com/problems/number-of-islands/
class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        vector<vector<bool>> visited(grid.size(), vector<bool>(grid[0].size(), false));
        int counter = 0;
        for (int i = 0; i < grid.size(); ++i) {
            for (int j = 0; j < grid[0].size(); ++j) {
                if (!visited[i][j] && grid[i][j] == '1') 
                    counter++;
                visit(visited, grid, i, j);
            }
        }
        return counter;
    }
    
    void visit(vector<vector<bool>>& visited, vector<vector<char>>& grid, int x, int y) {
        if (visited[x][y]) return;
        if (grid[x][y] == '0') return;
        
        visited[x][y] = true;
        if (x > 0 && grid[x-1][y] == '1') visit(visited, grid, x-1, y);
        if (y > 0 && grid[x][y-1] == '1') visit(visited, grid, x, y-1);
        if (x+1 < grid.size() && grid[x+1][y] == '1') visit(visited, grid, x+1, y);
        if (y+1 < grid[x].size() && grid[x][y+1] == '1') visit(visited, grid, x, y+1);
    }
};