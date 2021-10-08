// https://leetcode.com/problems/prefix-and-suffix-search/
struct Trie {
    int index;
    
    map<char, Trie*> prefix;
    map<char, Trie*> suffix;
    map<pair<char, char>, Trie*> both;
    
    Trie(int index) : index(index) {}
};

void add_prefix(Trie* root, string& s, int start, int index) {
    for (int i = start; i < s.size(); ++i) {
        auto prefix = s[i];
        if (root->prefix[prefix] == nullptr) {
            root->prefix[prefix] = new Trie(index);
        }
        
        root = root->prefix[prefix];
        root->index = max(root->index, index);
    }
}

void add_suffix(Trie* root, string& s, int start, int index) {
    for (int i = start; i < s.size(); ++i) {
        int i_rev = s.size() - 1 - i;
        auto suffix = s[i_rev];
        if (root->suffix[suffix] == nullptr) {
            root->suffix[suffix] = new Trie(index);
        }
        
        root = root->suffix[suffix];
        root->index = max(root->index, index);
    }
}

void add_both(Trie* root, string& s, int index) {
    for (int i = 0; i < s.size(); ++i) {
        int i_rev = s.size() - 1 - i;
        
        auto both = make_pair(s[i], s[i_rev]);
        if (root->both[both] == nullptr) {
            root->both[both] = new Trie(index);
        }
        
        add_prefix(root, s, i, index);
        add_suffix(root, s, i, index);
        
        root = root->both[both];
        root->index = max(root->index, index);
    }
}

class WordFilter {
public:
    Trie* trie;
    
    WordFilter(vector<string>& words) : trie(new Trie(0)) {
        for (int i = 0; i < words.size(); ++i) {
            add_both(this->trie, words[i], i);
        }
    }
    
    int f(string prefix, string suffix) {
        int i = 0;
        auto root = trie;
        for (; i < min(prefix.size(), suffix.size()); ++i) {
            auto both = make_pair(prefix[i], suffix[suffix.size()-1-i]);
            if (root->both[both] == nullptr) return -1;
            root = root->both[both];
        }
        if (prefix.size() > suffix.size()) {
            for (; i < prefix.size(); ++i) {
                if (root->prefix[prefix[i]] == nullptr) return -1;
                root = root->prefix[prefix[i]];
            }
        } else {
            for (; i < suffix.size(); ++i) {
                if (root->suffix[suffix[suffix.size()-1-i]] == nullptr) return -1;
                root = root->suffix[suffix[suffix.size()-1-i]];
            }
        }
        return root->index;
    }
};

/**
 * Your WordFilter object will be instantiated and called as such:
 * WordFilter* obj = new WordFilter(words);
 * int param_1 = obj->f(prefix,suffix);
 */


/* 

В Solution говорят что можно сделать попроще. Но я не буду делать, а просто возьму на заметку, ибо сложность такая же. Вот что они говорят:

Consider the word 'apple'. For each suffix of the word, we could insert that suffix, followed by '#', followed by the word, all into the trie.

For example, we will insert '#apple', 'e#apple', 'le#apple', 'ple#apple', 'pple#apple', 'apple#apple' into the trie. Then for a query like prefix = "ap", suffix = "le", we can find it by querying our trie for le#ap.

 */