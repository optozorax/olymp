// https://leetcode.com/problems/number-of-recent-calls/
class RecentCounter {
public:
    deque<int> counts;
    
    RecentCounter() {}
    
    int ping(int t) {
        counts.push_back(t);
        while (!counts.empty() && t - counts.front() > 3000) {
            counts.pop_front();
        }
        return counts.size();
    }
};

/**
 * Your RecentCounter object will be instantiated and called as such:
 * RecentCounter* obj = new RecentCounter();
 * int param_1 = obj->ping(t);
 */