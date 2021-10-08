// https://leetcode.com/problems/word-search-ii/
struct Trie {
    int index;
    unordered_map<char, Trie*> next;
    
    Trie(int index) : index(index) {}
};

void add(Trie* root, string& word, int index) {
    for (auto& i : word) {
        if (root->next[i] == nullptr) {
            root->next[i] = new Trie(-1);
        }
        
        root = root->next[i];
    }
    root->index = index;
}

class Solution {
public:
    int m = 0;
    int n = 0;
    
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        auto trie = new Trie(-1);
        for (int i = 0; i < words.size(); ++i) {
            add(trie, words[i], i);
        }
        
        this->m = board.size();
        this->n = board[0].size();

        unordered_set<int> result;
        for (int j = 0; j < m; ++j) {
            for (int i = 0; i < n; ++i) {
                dfs(board, trie, j, i, result);
            }
        }
        
        vector<string> results;
        for (auto& i : result) results.push_back(words[i]);
        return results;
    }

    void dfs(
        vector<vector<char>>& board, 
        Trie* trie, 
        int j, 
        int i,
        unordered_set<int>& result
    ) {
        char letter = board[j][i];
        if (trie->next[letter] != nullptr) {
            trie = trie->next[letter];
            board[j][i] = '#';
            if (trie->index != -1) result.insert(trie->index);
            if (j > 0) dfs(board, trie, j-1, i, result);
            if (i > 0) dfs(board, trie, j, i-1, result);
            if (j+1 < m) dfs(board, trie, j+1, i, result);
            if (i+1 < n) dfs(board, trie, j, i+1, result);
            board[j][i] = letter;
        }
    }
};