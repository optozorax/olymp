// https://leetcode.com/problems/course-schedule-ii/
class Solution {
public:
    vector<int> findOrder(int numCourses, vector<vector<int>>& preq) {
        unordered_set<int> unique;
        unordered_map<int, vector<int>> req;
        for (auto& i : preq) {
            unique.insert(i[0]);
            req[i[0]].push_back(i[1]);
        }
        
        unordered_set<int> visited;
        unordered_set<int> prev;
        unordered_map<int, int> pos;
        for (auto& i : unique) {
            if (!recur(req, prev, visited, pos, i)) 
                return vector<int>();
        }
        
        for (int i = 0; i < numCourses; ++i) pos[i] += 0;
        
        priority_queue<pair<int, int>> pq;
        for (auto& i : pos) pq.push(make_pair(-i.second, i.first));
        
        vector<int> result;
        while (pq.size() != 0) {
            result.push_back(pq.top().second);
            pq.pop();
        }
        return result;
    }
    
    bool recur(
        unordered_map<int, vector<int>>& req, 
        unordered_set<int>& previous,
        unordered_set<int>& visited,
        unordered_map<int, int>& pos, 
        int current
    ) {
        if (previous.find(current) != previous.end()) {
            return false;
        }
        if (visited.find(current) == visited.end()) {
            previous.insert(current);
            int best = 0;
            for (auto& i : req[current]) {
                if (!recur(req, previous, visited, pos, i)) 
                    return false;
                best = max(best, pos[i]);
            }
            visited.insert(current);
            pos[current] = best+1;
            previous.erase(previous.find(current));
        }
        return true;
    }
};