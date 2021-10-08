// https://leetcode.com/problems/count-of-range-sum/
struct SegmentTree {
    int n;
    vector<int> t;
    
    SegmentTree(vector<int>& a) : t(a.size() * 4), n(a.size()) {
        build(a, 1, 0, n-1);
    }
    
    void build(vector<int>& a, int v, int tl, int tr) {
        if (tl == tr) {
            t[v] = a[tl];
        } else {
            int tm = (tl + tr) / 2;
            build(a, v*2, tl, tm);
            build(a, v*2+1, tm+1, tr);
            t[v] = t[v*2] + t[v*2 + 1];
        }
    }
    
    int sum_inner(int v, int l, int r, int tl, int tr) {
        if (l > r) return 0;
        if (l == tl && r == tr) return t[v];
        int tm = (tl + tr) / 2;
        return
            sum_inner(v*2, l, min(r, tm), tl, tm) +
            sum_inner(v*2+1, max(tm+1, l), r, tm+1, tr);
    }
    
    int sum(int l, int r) {
        return sum_inner(1, l, r, 0, n-1);
    }
    
    void update_inner(int v, int tl, int tr, int pos, int new_val) {
        if (tl == tr) {
            t[v] = new_val;
        } else {
            int tm = (tl + tr) / 2;
            if (pos <= tm) {
                update_inner(v*2, tl, tm, pos, new_val);
            } else {
                update_inner(v*2+1, tm+1, tr, pos, new_val);
            }
            t[v] = t[v*2] + t[v*2 + 1];
        }
    }
    
    void update(int pos, int new_val) {
        update_inner(1, 0, n-1, pos, new_val);
    }
};

class Solution {
public:
    int countRangeSum(vector<int>& nums, int lower, int upper) {
        vector<int64_t> prefix(nums.size(), 0);
        for (int i = 0; i < nums.size(); ++i) {
            if (i == 0) {
                prefix[i] = nums[i];
            } else {
                prefix[i] = prefix[i-1] + nums[i];
            }
        }
        
        vector<pair<int64_t, int>> sorted_prefix;
        for (int i = 0; i < prefix.size(); ++i) {
            sorted_prefix.push_back(make_pair(prefix[i], i));
        }
        sort(sorted_prefix.begin(), sorted_prefix.end());
        
        vector<int> rev(nums.size());
        for (int i = 0; i < sorted_prefix.size(); ++i) {
            rev[sorted_prefix[i].second] = i;
        }
        
        vector<int> removed(nums.size(), 0);
        
        SegmentTree sg(removed);
        
        int result = 0;
        int64_t prev_sum = 0;
        for (int i = 0; i < nums.size(); ++i) {
            int lower_pos = binary_search2(0, sorted_prefix.size(), [&] (int c) {
                return sorted_prefix[c].first - prev_sum >= lower;
            });
            int upper_pos = binary_search2(0, sorted_prefix.size(), [&] (int c) {
                return sorted_prefix[c].first - prev_sum > upper;
            });
            
            int deleted = sg.sum(lower_pos, upper_pos-1);
            
            int count = upper_pos - lower_pos - deleted;
                
            removed[rev[i]] = 1;
            sg.update(rev[i], 1);
            
            result += count;
                
            prev_sum += nums[i];
        }
        return result;
    }
    
    template<typename F>
    int binary_search(int a, int b, F good) {
        int old_b = b;
        if (b-a == 0) return -1;
        if (good(a)) return a;
        if (!good(b-1)) return -1;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return -1;
    }
    
    template<typename F>
    int binary_search2(int a, int b, F good) {
        int result = binary_search(a, b, good);
        if (result == -1) return b;
        return result;
    }
};