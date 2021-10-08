// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
class Solution {
public:
    int kthSmallest(vector<vector<int>>& matrix, int k) {
        k--;
        int answer = 0;
        int n = matrix.size();
        binary_search2(matrix[0][0], matrix[n-1][n-1]+1, [&](int m) { 
            int sum1 = 0;
            int sum2 = 0;
            for (int i = 0; i < n; ++i) {
                sum1 += binary_search2(0, n, [&](int c) { return matrix[i][c] >= m; });
                sum2 += binary_search2(0, n, [&](int c) { return matrix[i][c] > m; });
            }
            if (sum1 <= k && k <= sum2) answer = m;
            return sum2 > k;
        });
        return answer;
    }
    
    template<typename F> // function<bool(int)>
    int binary_search2(int a, int b, F good) {
        int old_b = b;
        if (b-a == 0) return old_b;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return old_b;
    }
};