#include <vector>
#include <iostream>
#include <string>
#include <map>
#include <list>
#include <sstream>
#include <fstream>
#include <algorithm>

using namespace std;

int calcDaysOffset(const vector<int>& week, int lessonsToLearn, int lessonsAtWeek, int offset) {
	int result = 0;
	if (lessonsToLearn/lessonsAtWeek > 5) {
		result += (lessonsToLearn / lessonsAtWeek - 5)*7;
		lessonsToLearn -= (lessonsToLearn / lessonsAtWeek - 5) * lessonsAtWeek;
	}
	while (lessonsToLearn > 0) {
		if (week[offset] == 1)
			lessonsToLearn--;
		result++;
		offset = (offset + 1) % 7;
	}
	return result;
}

int calcDays(const vector<int>& week, int lessonsToLearn, int lessonsAtWeek) {
	int mymin = -1;
	for (int i = 0; i < week.size(); i++) {
		if (week[i] == 1) {
			auto res = calcDaysOffset(week, lessonsToLearn, lessonsAtWeek, i);
			if (mymin == -1) mymin = res;
			else mymin = min(mymin, res);
		}
	}
	return mymin;
}

int main() {
	int n, lessonsToLearn, lessonsAtWeek;
	cin >> n;

	for (int i = 0; i < n; ++i) {
		lessonsAtWeek = 0;
		cin >> lessonsToLearn;
		vector<int> week(7);
		for (int i = 0; i < 7; ++i) {
			cin >> week[i];
			if (week[i] == 1)
				++lessonsAtWeek;
		}

		cout << calcDays(week, lessonsToLearn, lessonsAtWeek) << endl;
	}
}