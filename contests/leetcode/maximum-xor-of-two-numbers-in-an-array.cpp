// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
class Solution {
public:
    struct Trie {
        int8_t index;
        Trie* zero;
        Trie* one;
        
        Trie(int index) : index(index), zero(nullptr), one(nullptr) {}
    };
    
    void add(Trie* root, int num) {
        for (int8_t pos = 31; pos >= 0; --pos) {
            int8_t current = (num >> pos) % 2;
            if (current == 0) {
                if (root->zero == nullptr) root->zero = new Trie(pos);
                root = root->zero;
            } else {
                if (root->one == nullptr) root->one = new Trie(pos);
                root = root->one;
            }
        }
    }
    
    int findMaximumXOR(vector<int>& nums) {
        Trie* root = new Trie(32);
        for (const auto& i : nums) add(root, i);
        
        uint32_t best = 0;
        uint32_t current = ~0;
        recur(root, root, current, best);
        return best;
    }
    
    void recur(Trie* one, Trie* zero, uint32_t current, uint32_t& best) {
        if (current < best) return;
        
        if (one->one == nullptr && one->zero == nullptr) {
            best = max(best, current);
            return;
        }

        if (one->one != nullptr && zero->zero != nullptr) {
            recur(one->one, zero->zero, current, best);
        }
        if (one->zero != nullptr && zero->one != nullptr) {
            recur(one->zero, zero->one, current, best);
        }
        if (one->one != nullptr && zero->one != nullptr) {
            recur(one->one, zero->one, current & ~(1 << one->one->index) , best);
        }
        if (one->zero != nullptr && zero->zero != nullptr) {
            recur(one->zero, zero->zero, current & ~(1 << one->zero->index) , best);
        }
    }
};