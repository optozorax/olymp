// https://leetcode.com/problems/insert-delete-getrandom-o1/
class RandomizedSet {
public:
    unordered_map<int, int> s; // get index by val
    vector<int> v; // get val by index
    mt19937 g;
    
    RandomizedSet() : g(0) {}
    
    bool insert(int val) {
        if (s.find(val) != s.end()) return false;
        
        s[val] = v.size();
        v.push_back(val);
        return true;
    }
    
    bool remove(int val) {
        if (s.find(val) == s.end()) return false;
        
        auto index = s[val];
        v[index] = v.back();
        s[v.back()] = index;
        v.pop_back();
        s.erase(val);
        return true;
    }
    
    int getRandom() {
        uniform_int_distribution<int> d(0, v.size()-1);
        return v[d(g)];
    }
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */
