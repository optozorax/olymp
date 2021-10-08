// https://leetcode.com/problems/combinations/
class Solution {
public:
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> result;
        vector<int> current(k, 0);
        recur(n, k, 0, 0, current, result);
        return result;
    }
    
    void recur(
        int n, 
        int k, 
        int pos, 
        int index, 
        vector<int>& current, 
        vector<vector<int>>& result
    ) {
        if (index == k) {
            result.push_back(current);
        } else  {
            for (int i = pos; i < n-(k-index-1); ++i) {
                current[index] = i + 1;
                recur(n, k, i + 1, index + 1, current, result);
            }
        }
    }
};