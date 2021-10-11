// https://leetcode.com/problems/spiral-matrix-ii/
class Solution {
public:
    vector<vector<int>> generateMatrix(int n) {
        vector<vector<int>> res(n, vector<int>(n, 0));
        
        vector<int> state_x = {1, 0, -1, 0};
        vector<int> state_y = {0, 1, 0, -1};
        int state = 0;
        int x = 0;
        int y = 0;
        for (int i = 0; i < n*n; ++i) {
            res[y][x] = i+1;
            if (
                x + state_x[state] == n || 
                x + state_x[state] == -1 || 
                y + state_y[state] == n || 
                y + state_y[state] == -1 ||
                res[y + state_y[state]][x + state_x[state]] != 0
            ) {
                state = (state + 1) % 4;
            }
            x += state_x[state];
            y += state_y[state];
        }
        return res;
    }
};