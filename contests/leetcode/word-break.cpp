// https://leetcode.com/problems/word-break/
class Solution {
public:
    struct Trie {
        bool is_key;
        map<char, Trie*> next;

        Trie() : is_key(false), next() {}
    };

    void add(Trie* current, string& word) {
        for (int i = 0; i < word.size(); ++i) {
            if (current->next[word[i]] == nullptr) {
                current->next[word[i]] = new Trie();
            }
            current = current->next[word[i]];
        }
        current->is_key = true;
    }
    
    bool wordBreak(string s, vector<string>& wordDict) {
        Trie* trie = new Trie();
        for (auto& i : wordDict) {
            add(trie, i);
        }

        vector<bool> dp(s.size() + 1, false);
        dp[0] = true;

        for (int i = 0; i < s.size(); ++i) {
            if (dp[i]) {
                auto count = 0;
                auto j = trie;
                while (j != nullptr && i + count < s.size() + 1) {
                    if (j->is_key) {
                        dp[i + count] = true;
                    }
                    j = j->next[s[i + count]];
                    count++;
                }
            }
        }

        return dp.back();
    }
};