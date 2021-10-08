// https://leetcode.com/problems/set-matrix-zeroes/
class Solution {
public:
    void setZeroes(vector<vector<int>>& matrix) {
        bool first_col_has_zero = false;
        bool first_row_has_zero = false;
        for (int j = 0; j < matrix[0].size(); ++j) { // cols
            first_row_has_zero |= matrix[0][j] == 0;
        }
        for (int i = 0; i < matrix.size(); ++i) { // rows
            first_col_has_zero |= matrix[i][0] == 0;
        }
        for (int i = 1; i < matrix.size(); ++i) { // rows
            for (int j = 1; j < matrix[0].size(); ++j) { // cols
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for (int i = 1; i < matrix.size(); ++i) { // rows
            if (matrix[i][0] == 0) {
                for (int j = 1; j < matrix[0].size(); ++j) { // cols
                    matrix[i][j] = 0;
                }
            }
        }
        for (int j = 1; j < matrix[0].size(); ++j) { // cols
            if (matrix[0][j] == 0) {
                for (int i = 1; i < matrix.size(); ++i) { // rows
                    matrix[i][j] = 0;
                }
            }
        }
        if (first_col_has_zero) {
            for (int i = 0; i < matrix.size(); ++i) { // rows
                matrix[i][0] = 0;
            }
        }
        if (first_row_has_zero) {
            for (int j = 0; j < matrix[0].size(); ++j) { // cols
                matrix[0][j] = 0;
            }
        }
    }
};