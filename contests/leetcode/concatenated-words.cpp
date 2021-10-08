// https://leetcode.com/problems/concatenated-words/
struct Trie {
    bool end;
    unordered_map<char, Trie*> next;
    
    Trie() : end(false) {}
};

void add(Trie* root, string& word) {
    for (auto& i : word) {
        if (root->next[i] == nullptr) {
            root->next[i] = new Trie();
        }
        
        root = root->next[i];
    }
    root->end = true;
}

bool search_copy(Trie* root, string& s, int start) {
    auto root_copy = root;
    for (int i = start; i < s.size(); ++i) {
        if (root->next[s[i]] == nullptr) {
            return false;
        }
        
        root = root->next[s[i]];
        
        if (i != s.size() - 1 && root->end) {
            if (search_copy(root_copy, s, i+1)) {
                return true;
            }
        }
    }
    return root->end;
}

class Solution {
public:
    vector<string> findAllConcatenatedWordsInADict(vector<string>& words) {
        sort(words.begin(), words.end(), [] (const auto& l, const auto& r) {
            return l.size() < r.size();
        });
        auto trie = new Trie();
        vector<string> result;
        for (auto& i : words) {
            if (!search_copy(trie, i, 0)) {
                add(trie, i);
            } else {
                result.push_back(i);
            }
        }
        return result;
    }
};