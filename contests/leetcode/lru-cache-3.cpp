// https://leetcode.com/problems/lru-cache/
class LRUCache {
public:
    unordered_map<int, int> v;
    unordered_map<int, int16_t> p;
    
    struct ListElem { int val; int16_t prev; int16_t next; };
    vector<ListElem> l;
    
    LRUCache(int capacity) {
        l.reserve(capacity + 2);
        l.push_back(ListElem{-1, int16_t(capacity+2), 1});
        for (int16_t i = 0; i < capacity; ++i) 
            l.push_back(ListElem{0, i, int16_t(i+2)});
        l.push_back(ListElem{-1, int16_t(capacity), int16_t(capacity+2)});
    }
    
    void del(size_t pos) {
        auto elem = l[pos];
        l[elem.prev].next = elem.next;
        l[elem.next].prev = elem.prev;
        l[pos].next = l.size()-1;
        l[pos].prev = l.back().prev;
        l[l.back().prev].next = pos;
        l.back().prev = pos;
    }
    
    void push_front(int val) {
        auto prev = l.back().prev;
        auto prev_prev = l[prev].prev;
        l.back().prev = prev_prev;
        l[prev_prev].next = l.size()-1;
        
        l[prev].next = l[0].next;
        l[prev].prev = 0;
        l[prev].val = val;
        
        l[l[0].next].prev = prev;
        l[0].next = prev;
    }
    
    int pop() {
        return l[l.back().prev].val;
    }
    
    int begin() {
        return l[0].next;
    }
    
    int get(int key) {
        if (v.find(key) == v.end()) return -1;
        
        del(p[key]);
        push_front(key);
        p[key] = begin();
        
        return v[key];
    }
    
    void put(int key, int value) {
        if (v.find(key) == v.end()) {
            if (v.size() == l.size() - 2) {
                auto to_delete = pop();
                v.erase(to_delete);
                p.erase(to_delete);
            }
        } else {
            del(p[key]);
        }
        
        v[key] = value;
        push_front(key);
        p[key] = begin();
    }
};