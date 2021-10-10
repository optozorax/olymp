// https://leetcode.com/problems/lru-cache/
class LRUCache {
public:
    int64_t counter;
    int capacity;
    unordered_map<int, int> v;
    unordered_map<int, list<int>::iterator> p;
    list<int> q;
    
    LRUCache(int capacity) : capacity(capacity), counter(0) {}
    
    int get(int key) {
        if (v.find(key) == v.end()) return -1;
        
        q.erase(p[key]);
        q.push_front(key);
        p[key] = q.begin();
        
        return v[key];
    }
    
    void put(int key, int value) {
        if (v.find(key) == v.end()) {
            if (v.size() == capacity) {
                auto to_delete = q.back(); q.pop_back();
                v.erase(to_delete);
                p.erase(to_delete);
            }
        } else {
            q.erase(p[key]);
        }
        
        v[key] = value;
        q.push_front(key);
        p[key] = q.begin();
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */