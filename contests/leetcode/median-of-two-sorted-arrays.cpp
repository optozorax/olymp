// https://leetcode.com/problems/median-of-two-sorted-arrays/
class Solution {
public:
    template<typename F> // F is lambda(int) -> bool
    int binary_search(int a, int b, F good) {
        int old_b = b;
        if (b-a == 0) return -1;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return -1;
    }
    
    pair<int, bool> find_by_pos(vector<int>& a, vector<int>& b, int pos) {
        int answer = 0;
        bool found = false;
        binary_search(0, a.size(), [&](int i){
            int result1 = binary_search(0, b.size(), [&](int j){ return b[j] >= a[i]; });
            int result2 = binary_search(0, b.size(), [&](int j){ return b[j] >  a[i]; });

            if (result1 == -1) result1 = b.size();
            if (result2 == -1) result2 = b.size();
            
            if (i + result1 <= pos && pos <= i + result2) {
                answer = a[i];
                found = true;
            }

            return result2 + i >= pos;
        });
        return {answer, found};
    }
    
    int find_by_pos2(vector<int>& a, vector<int>& b, int pos) {
        auto res = find_by_pos(a, b, pos);
        if (!res.second) {
            return find_by_pos(b, a, pos).first;
        } else {
            return res.first;
        }
    }
    
    double findMedianSortedArrays(vector<int>& a, vector<int>& b) {
        int n = a.size() + b.size();
        if (n % 2 == 1) {
            return double(find_by_pos2(a, b, n / 2));
        } else {
            return 
                (double(find_by_pos2(a, b, n / 2 - 1)) + 
                double(find_by_pos2(a, b, n / 2))) / 2.0;
        }
    }
};