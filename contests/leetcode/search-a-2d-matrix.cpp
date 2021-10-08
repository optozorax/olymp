// https://leetcode.com/problems/search-a-2d-matrix/
class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int m = matrix.size();
        int n = matrix[0].size();
        int pos = binary_search(0, m * n, [&](int c){
            return matrix[c / n][c % n] >= target;
        });
        
        if (pos == -1) return false;
        return matrix[pos / n][pos % n] == target;
    }
    
    template<typename F> // function<bool(int)>
    int binary_search(int a, int b, F good) {
        int old_b = b;
        if (b-a == 0) return -1;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return -1;
    }
};