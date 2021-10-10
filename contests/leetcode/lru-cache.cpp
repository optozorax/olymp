// https://leetcode.com/problems/lru-cache/
class LRUCache {
public:
    int64_t counter;
    int capacity;
    unordered_map<int, pair<int, int64_t>> m;
    map<int64_t, int> priority;
    
    LRUCache(int capacity) : capacity(capacity), counter(0) {}
    
    int get(int key) {
        if (m.find(key) == m.end()) return -1;
        
        update(key, m[key].first);
        
        return m[key].first;
    }
    
    void put(int key, int value) {
        if (m.find(key) == m.end()) {
            if (m.size() == capacity) {
                auto to_delete = (*priority.begin()).second;
                priority.erase(priority.begin());
                m.erase(to_delete);
            }
        }
        
        update(key, value);
    }
    
    void update(int key, int value) {
        if (m.find(key) != m.end()) 
            priority.erase(m[key].second);
        
        m[key] = make_pair(value, counter);
        priority[counter] = key;
        counter++;
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */