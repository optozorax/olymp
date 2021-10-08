// https://leetcode.com/problems/search-a-2d-matrix-ii/
class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        if (matrix.size() < matrix[0].size()) {
            for (int i = 0; i < matrix.size(); ++i) {
                int answer = binary_search(0, matrix[0].size(), [&](int c) { 
                    return matrix[i][c] >= target; 
                });
                if (answer != -1 && matrix[i][answer] == target) return true;
            }
        } else {
            for (int i = 0; i < matrix[0].size(); ++i) {
                int answer = binary_search(0, matrix.size(), [&](int c) { 
                    return matrix[c][i] >= target; 
                });
                if (answer != -1 && matrix[answer][i] == target) return true;
            }
        }
        return false;
    }
    
    template<typename F> // function<bool(int)>
    int binary_search(int a, int b, F good) {
        int old_b = b;
        if (b-a == 0) return -1;
        if (good(a)) return a;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return -1;
    }
};