// https://leetcode.com/problems/course-schedule/
class Solution {
public:
    bool canFinish(int numCourses, vector<vector<int>>& preq) {
        unordered_set<int> unique;
        unordered_map<int, vector<int>> req;
        for (auto& i : preq) {
            unique.insert(i[0]);
            req[i[0]].push_back(i[1]);
        }
        
        unordered_set<int> visited;
        unordered_set<int> prev;
        for (auto& i : unique) {
            if (!recur(req, prev, visited, i)) 
                return false;
        }
        return true;
    }
    
    bool recur(
        unordered_map<int, vector<int>>& req, 
        unordered_set<int>& previous,
        unordered_set<int>& visited,
        int current
    ) {
        if (previous.find(current) != previous.end()) {
            return false;
        }
        if (visited.find(current) == visited.end()) {
            previous.insert(current);
            for (auto& i : req[current]) {
                if (!recur(req, previous, visited, i)) 
                    return false;
            }
            visited.insert(current);
            previous.erase(previous.find(current));
        }
        return true;
    }
};