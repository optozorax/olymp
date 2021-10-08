// https://leetcode.com/problems/n-queens/
class Solution {
public:
    vector<vector<string>> solveNQueens(int n) {
        auto perms = permute(n);
        vector<vector<string>> result;
        for (auto& i : perms) {
            vector<string> solution;
            for (auto& j : i) {
                string current;
                for (int k = 0; k < n; ++k) {
                    if (k == j) {
                        current.push_back('Q');
                    } else {
                        current.push_back('.');
                    }
                }
                solution.push_back(current);
            }
            result.push_back(solution);
        }
        return result;
    }
    
    vector<vector<int>> permute(int size) {
        vector<vector<int>> result;
        vector<int> nums;
        for (int i = 0; i < size; ++i) nums.push_back(i);
        recur(nums, nums.size(), 0, result);
        return result;
    }
    
    void recur(vector<int>& current, int count, int index, vector<vector<int>>& result) {
        if (index > 1) {
            int beats1 = current[index-1] - 1;
            int beats2 = current[index-1] + 1;
            for (int i = index - 2; i >= 0; --i) {
                if (current[i] == beats1 || current[i] == beats2) return;
                beats1--;
                beats2++;
            }
        }
        if (index == current.size()) {
            result.push_back(current);
        } else {
            for (int i = 0; i < count; ++i) {
                swap(current[index], current[index+i]);
                recur(current, count-1, index+1, result);
                swap(current[index], current[index+i]);
            }
        }
    }
};