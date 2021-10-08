// https://leetcode.com/problems/implement-trie-prefix-tree/
struct MyTrie {
    bool is_key;
    map<char, MyTrie*> next;

    MyTrie() : is_key(false), next() {}
};

void add(MyTrie* current, string& word) {
    for (int i = 0; i < word.size(); ++i) {
        if (current->next[word[i]] == nullptr) {
            current->next[word[i]] = new MyTrie();
        }
        current = current->next[word[i]];
    }
    current->is_key = true;
}

class Trie {
public:
    MyTrie* inner;
    
    Trie() {
        inner = new MyTrie();
    }
    
    void insert(string word) {
        add(inner, word);
    }
    
    bool search(string word) {
        auto j = inner;
        for (int i = 0; i < word.size() && j != nullptr; ++i) {
            j = j->next[word[i]];
        }
        if (j == nullptr) return false;
        
        return j->is_key;
    }
    
    bool startsWith(string prefix) {
        auto j = inner;
        for (int i = 0; i < prefix.size() && j != nullptr; ++i) {
            j = j->next[prefix[i]];
        }
        return j != nullptr;
    }
};

/**
 * Your Trie object will be instantiated and called as such:
 * Trie* obj = new Trie();
 * obj->insert(word);
 * bool param_2 = obj->search(word);
 * bool param_3 = obj->startsWith(prefix);
 */