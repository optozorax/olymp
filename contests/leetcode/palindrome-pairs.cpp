// https://leetcode.com/problems/palindrome-pairs/
struct Trie {
    vector<int> indexes; // from here to end, these substrings are palindromes
    vector<int> ended;
    unordered_map<char, Trie*> next;
    
    Trie() {}
};

// Manacher algorithm to get all palindromes
vector<int> manacher(string& s) {
    auto get_s = [&s](int pos) {
        return pos % 2 == 1 ? '#' : s[pos / 2];
    };
    int n = s.size() * 2 - 1;
    vector<int> z(n, 0);
    int l = 0;
    int r = 0;
    for (int i = 0; i < n; ++i) {
        if (i <= r) z[i] = min(r-i, z[l+r-i]);
        while (
            i + z[i] + 1 > r && 
            i + z[i] + 1 < n && 
            i - z[i] - 1 >= 0 && 
            get_s(i + z[i] + 1) == get_s(i - z[i] - 1)
        ) {
            z[i]++;
        }
        if (i + z[i] > r) {
            r = i + z[i];
            l = i - z[i];
        }
    }
    return z;
}

// maximal palindrome diameter in the position I (odd I represent between letters)
int diameter(vector<int>& z, int i) {
    return ((z[i] + i % 2) / 2) * 2 + (1 - i % 2);
}

// s[i..=j] is palindrome
bool manacher_is_palindrome(vector<int>& z, int i, int j) {
    int size = j-i+1;
    int c = i+j;
    int d = diameter(z, c);
    return size <= d;
}

void add(Trie* root, string& s, vector<int>& z, int index) {
    for (int i = 0; i < s.size(); ++i) {
        if (root->next[s[i]] == nullptr) 
            root->next[s[i]] = new Trie();
        
        root = root->next[s[i]];
        if (i == s.size() - 1 || manacher_is_palindrome(z, i+1, s.size()-1)) {
            root->indexes.push_back(index);
        }
    }
    root->ended.push_back(index);
}

class Solution {
public:
    vector<vector<int>> palindromePairs(vector<string>& words) {
        vector<vector<int>> zs(words.size());
        auto trie = new Trie();
        for (int i = 0; i < words.size(); ++i) {
            if (!words[i].empty()) {
                auto z = manacher(words[i]);
                add(trie, words[i], z, i);
                zs[i] = z;
            }
        }
        
        vector<vector<int>> result;
        for (int i = 0; i < words.size(); ++i) {
            auto& word = words[i];
            if (word.empty()) {
                for (int j = 0; j < words.size(); ++j) {
                    if (i != j && (words[j].empty() || manacher_is_palindrome(zs[j], 0, words[j].size()-1))) {
                        result.push_back({i, j});
                        result.push_back({j, i});
                    }
                }
            } else {
                int l = word.size();
                int w = l-1;
                auto t = trie;
                while (w != -1 && t->next[word[w]] != nullptr) {
                    t = t->next[word[w]];
                    w--;

                    if (w == -1) {
                        for (auto& j : t->indexes) {
                            if (j != i)
                                result.push_back({j, i});
                        }
                    } else {
                        if (manacher_is_palindrome(zs[i], 0, w)) {
                            for (auto& j : t->ended) {
                                if (j != i)
                                    result.push_back({j, i});
                            }
                        }
                    }
                }
            }
        }
        
        return result;
    }
};