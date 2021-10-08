// https://leetcode.com/problems/course-schedule-iii/
class Solution {
public:
    int scheduleCourse(vector<vector<int>>& courses) {
        sort(courses.begin(), courses.end(), [] (const auto& l, const auto& r) {
            return l[1] < r[1];
        });
        
        int time = 0;
        priority_queue<int> taken;
        for (int i = 0; i < courses.size(); ++i) {
            if (time + courses[i][0] <= courses[i][1]) {
                time += courses[i][0];
                taken.push(courses[i][0]);
            } else {
                if (
                    !taken.empty() && 
                    time - taken.top() + courses[i][0] <= time &&
                    time - taken.top() + courses[i][0] <= courses[i][1]
                ) {
                    time -= taken.top();
                    time += courses[i][0];
                    taken.pop();
                    taken.push(courses[i][0]);
                }
            }
        }
        return taken.size();
    }
};

/* 

неправильно решение, пытался сделать как задача о рюкзаке, но time limit exceeded, зато лучше проникся рюкзаком. это решение падает по TLE, но в целом работает правильно

class Solution {
public:
    int scheduleCourse(vector<vector<int>>& courses) {
        sort(courses.begin(), courses.end(), [] (const auto& l, const auto& r) {
            return l[1] < r[1];
        });
        
        int day_max = courses.back()[1];
        int n = courses.size();
        
        vector<vector<int>> a(n+1, vector<int>(day_max+1, 0));
        
        int best = 0;
        for (int k = 1; k <= n; ++k) {
            for (int s = 1; s <= day_max; ++s) {
                if (courses[k-1][0] <= s && s <= courses[k-1][1]) {
                // if (courses[k-1][0] <= s) { // для такой строчки это будет классический рюкзак
                    a[k][s] = max(a[k-1][s], a[k-1][s-courses[k-1][0]] + 1);
                } else {
                    a[k][s] = a[k-1][s];
                }
                best = max(best, a[k][s]);
            }
        }
     
        return best;
    }
};

 */