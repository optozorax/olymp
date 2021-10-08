// https://leetcode.com/problems/task-scheduler/
class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        int all = 0;
        vector<int> count(27);
        for (auto& i : tasks) {
            count[i-'A']++;
            all++;
        }
        int answer = 0;
        while (true) {
            sort(count.begin(), count.end(), [] (const auto& l, const auto& r) {
                return l > r;
            });
            int needed = n+1;
            for (int i = 0; i < count.size(); ++i) {
                if (count[i] != 0) {
                    count[i]--;
                    answer++;
                    needed--;
                    all--;
                    if (all == 0) 
                        return answer;
                    if (needed == 0) 
                        break;
                }
            }
            answer += needed;
        }
        return answer;
    }
};