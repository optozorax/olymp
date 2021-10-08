// https://leetcode.com/problems/reorganize-string/
class Solution {
public:
    string reorganizeString(string s) {
        map<char, int> m;
        for (auto& i : s) m[i]++;
        
        priority_queue<pair<int, char>> p;
        for (auto& i : m) p.push(make_pair(i.second, i.first));
        
        string res;
        while (p.size() != 0) {
            auto top1 = p.top(); p.pop();
            
            if (p.size() == 0) {
                if (top1.first == 1) {
                    res.push_back(top1.second);
                    return res;
                } else {
                    return string();
                }
            }
            auto top2 = p.top(); p.pop();
            
            res.push_back(top1.second);
            res.push_back(top2.second);
            
            if (top1.first-1 > 0) p.push(make_pair(top1.first - 1, top1.second));
            if (top2.first-1 > 0) p.push(make_pair(top2.first - 1, top2.second));
        }
        return res;
    }
};