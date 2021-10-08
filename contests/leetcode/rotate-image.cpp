// https://leetcode.com/problems/rotate-image/
class Solution {
public:
    void rotate(vector<vector<int>>& matrix) {
        int n = matrix.size();
        for (int i = 0; i < n / 2; ++i) {
            int size = n-1-i*2;
            for (int j = 0; j < size; ++j) {
                swap(matrix[i+0][i+j], matrix[i+j][i+size]);
                swap(matrix[i+0][i+j], matrix[i+size-j][i+0]);
                swap(matrix[i+size-j][i+0], matrix[i+size][i+size-j]);
            }
        }
    }
};