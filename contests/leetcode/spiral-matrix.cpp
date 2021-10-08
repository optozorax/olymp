// https://leetcode.com/problems/spiral-matrix/
class Solution {
public:
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        int m = matrix.size();
        int n = matrix[0].size();
        const array<int, 4> x_order = {1, 0, -1, 0};
        const array<int, 4> y_order = {0, 1, 0, -1};
        int x_pos = 0;
        int y_pos = 0;
        int state = 0;
        
        auto out = [&](int x, int y) {
            return
                x < 0 || y < 0 ||
                x >= n || y >= m ||
                matrix[y][x] == -1000;            
        };
        
        vector<int> result;
        result.reserve(n * m);
        for (int i = 0; i < n * m; ++i) {
            result.push_back(matrix[y_pos][x_pos]);
            matrix[y_pos][x_pos] = -1000;
            for (int i = 0; i < 4; ++i) {
                if (!out(x_pos + x_order[(state + i) % 4], y_pos + y_order[(state + i) % 4])) {
                    state = (state + i) % 4;
                    x_pos = x_pos + x_order[state];
                    y_pos = y_pos + y_order[state];
                    break;
                }
            }
        }

        return result;
    }
};