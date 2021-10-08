// https://leetcode.com/problems/meeting-rooms/
/**
 * Definition of Interval:
 * classs Interval {
 *     int start, end;
 *     Interval(int start, int end) {
 *         this->start = start;
 *         this->end = end;
 *     }
 * }
 */

class Solution {
public:
    /**
     * @param intervals: an array of meeting time intervals
     * @return: if a person could attend all meetings
     */
    bool canAttendMeetings(vector<Interval> &intervals) {
        if (intervals.size() == 0) return true;
        sort(intervals.begin(), intervals.end(), [](const auto& l, const auto& r){ 
            return l.start < l.start;
        });
        for (int i = 0; i < intervals.size() - 1; ++i) {
            if (intervals[i].end > intervals[i+1].start) return false;
        }
        return true;
    }
};