// https://leetcode.com/problems/find-median-from-data-stream/
class MedianFinder {
public:
    multiset<int> l, r;
    
    MedianFinder() : l(), r() {
        
    }
    
    void addNum(int num) {
        if (l.size() == 0) {
            l.insert(num);
        } else {
            if (num > *l.rbegin()) {
                r.insert(num);
                if (r.size() > l.size()) {
                    l.insert(*r.begin());
                    r.erase(r.find(*r.begin()));
                }
            } else {
                l.insert(num);
                if (l.size() - r.size() > 1) {
                    r.insert(*l.rbegin());
                    l.erase(l.find(*l.rbegin()));
                }
            }
        }
    }
    
    double findMedian() {
        if ((l.size() + r.size()) % 2 == 1) {
            return double(*l.rbegin());
        } else {
            return (double(*l.rbegin()) + double(*r.begin())) / 2.;
        }
    }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */