// https://leetcode.com/problems/word-search/
class Solution {
public:
    int m = 0;
    int n = 0;

    bool exist(vector<vector<char>>& board, string word) {
        this->m = board.size();
        this->n = board[0].size();

        for (int j = 0; j < m; ++j) {
            for (int i = 0; i < n; ++i) {
                if (dfs(board, word, 0, j, i)) 
                    return true;
            }
        }
        return false;
    }

    bool dfs(vector<vector<char>>& board, string& word, int index, int j, int i) {
        if (board[j][i] == word[index]) {
            board[j][i] = '#';
            if (index+1 == word.size()) return true;
            if (j > 0 && dfs(board, word, index+1, j-1, i)) return true;
            if (i > 0 && dfs(board, word, index+1, j, i-1)) return true;
            if (j+1 < m && dfs(board, word, index+1, j+1, i)) return true;
            if (i+1 < n && dfs(board, word, index+1, j, i+1)) return true;
            board[j][i] = word[index];
        }
        return false;
    }
};