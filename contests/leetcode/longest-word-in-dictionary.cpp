// https://leetcode.com/problems/longest-word-in-dictionary/
class Solution {
public:
    struct Trie {
        bool word_end;
        map<char, Trie*> next;
        
        Trie() : word_end(false) {}
    };
    
    void add(Trie* root, string s) {
        for (auto& i : s) {
            if (root->next[i] == nullptr) {
                root->next[i] = new Trie();
            }
            
            root = root->next[i];
        }
        root->word_end = true;
    }
    
    string longestWord(vector<string>& words) {
        auto trie = new Trie();
        trie->word_end = true;
        for (auto& i : words) add(trie, i);
        
        string best;
        string current;
        recur(trie, current, best);
        return best;
    }
    
    void recur(Trie* root, string& current, string& best) {
        if (root == nullptr) return;
        
        if (root->word_end) {
            if (current.size() > best.size() || (current.size() == best.size() && current < best))
                best = current;
        } else {
            return;
        }
        
        for (auto& i : root->next) {
            current.push_back(i.first);
            recur(i.second, current, best);
            current.pop_back();
        }
    }
};