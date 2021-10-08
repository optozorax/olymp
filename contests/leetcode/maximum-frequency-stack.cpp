// https://leetcode.com/problems/maximum-frequency-stack/
class FreqStack {
public:
    priority_queue<tuple<int, int, int>> pq;
    unordered_map<int, int> count;
    int index;
    
    FreqStack() : index(0) {
        
    }
    
    void push(int val) {
        index++;
        count[val]++;
        pq.push(make_tuple(count[val], index, val));
    }
    
    int pop() {
        auto [freq, index, val] = pq.top(); pq.pop();
        count[val]--;
        return val;
    }
};

/**
 * Your FreqStack object will be instantiated and called as such:
 * FreqStack* obj = new FreqStack();
 * obj->push(val);
 * int param_2 = obj->pop();
 */