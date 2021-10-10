// https://leetcode.com/problems/perfect-squares/
class Solution {
public:
    int numSquares(int n) {
        static vector<int> squares;
        static vector<int> count;
        
        squares.clear();
        count.resize(n+2, n+2);
        
        for (int i = 1; i*i <= n; ++i) {
            squares.push_back(i*i);
            count[i*i] = 1;
        }
        
        for (int i = 1; i <= n; ++i) {
            int j = 0;
            while (j < squares.size() && i + squares[j] <= n) {
                count[i + squares[j]] = min(count[i + squares[j]], count[i] + 1);
                j++;
            }
        }
        
        return count[n];
    }
};