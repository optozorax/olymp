// https://leetcode.com/problems/substring-with-concatenation-of-all-words/
struct Trie {
    int index;
    unordered_map<char, Trie*> next;
    
    Trie() {}
};

void add(Trie* root, string& word, int index) {
    for (auto& i : word) {
        if (root->next[i] == nullptr) {
            root->next[i] = new Trie();
        }
        
        root = root->next[i];
    }
    root->index = index;
}

int search(Trie* root, string& s, int start, int end) {
    for (int i = start; i < end; ++i) {
        if (root->next[s[i]] == nullptr) {
            return -1;
        }
        
        root = root->next[s[i]];
    }
    return root->index;
}

int search_easy(Trie* root, string& word) {
    return search(root, word, 0, word.size());
}

class Solution {
public:
    vector<int> findSubstring(string s, vector<string>& words) {
        int n = s.size();
        int k = words[0].size();
        int wc = words.size();
        
        vector<int> words_count;
        auto trie = new Trie();
        for (auto& i : words) {
            int found = search_easy(trie, i);
            if (found == -1) {
                add(trie, i, words_count.size());
                words_count.push_back(1);
            } else {
                words_count[found]++;
            }
        }
        
        vector<int> indexes;
        for (int i = 0; i < s.size() - k + 1; ++i) {
            indexes.push_back(search(trie, s, i, i + k)); 
        }
        
        vector<int> result;
        for (int i = 0; i < k; ++i) {
            vector<int> my_count(words_count.size(), 0);
            int same = 0;
            int r = 0;
            int end = (n - i) / k;
            for (int l = 0; l <= end - wc; ++l) {
                while (r <= end && r < l + wc) {
                    int pos = indexes[i + r*k];
                    if (pos != -1) {
                        my_count[pos]++;
                        if (my_count[pos] == words_count[pos]) same++;
                    }
                    r++;
                }
                if (same == words_count.size()) {
                    result.push_back(i + l*k);
                }
                int pos = indexes[i + l*k];
                if (pos != -1) {
                    if (my_count[pos] == words_count[pos]) same--;
                    my_count[pos]--;
                }
            }
        }
        return result;
    }
};