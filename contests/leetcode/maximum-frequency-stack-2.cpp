// https://leetcode.com/problems/maximum-frequency-stack/
class FreqStack {
public:
    vector<vector<int>> group;
    unordered_map<int, int> count;
    int max_freq;
    
    FreqStack() : max_freq(0) {
        group.push_back({});
    }
    
    void push(int val) {
        count[val]++;
        max_freq = max(max_freq, count[val]);
        if (group.size() <= max_freq) group.push_back({});
        group[count[val]].push_back(val);
    }
    
    int pop() {
        int val = group[max_freq].back(); group[max_freq].pop_back();
        if (group[max_freq].empty()) max_freq--;
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