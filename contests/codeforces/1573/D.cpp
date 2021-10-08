#include <algorithm>
#include <iostream>
#include <vector>
#include <set>
#include <queue>
#include <cmath>
 
using namespace std;

void debug_arr(string name, vector<int> a) {
	cout << "| ";
	cout << name << ": ";
	for (int i = 0; i < a.size(); ++i) {
		cout << a[i] << " ";
	}
	cout << endl;
}

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int n;
		cin >> n;

		vector<int> a(n);
		for (int i = 0; i < n; ++i) {
			cin >> a[i];
		}

		deque<int> to_visit;

		vector<int> count(n, 0);
		for (int i = 0; i < n-2; ++i) {
			count[i] = a[i] + a[i+1] + a[i+2];
			if (count[i] == 2) {
				to_visit.push_front(i);
			}
		}
		count[n-2] = a[n-2];
		count[n-1] = a[n-1];

		// debug_arr("count", count);

		vector<int> result;

		while (to_visit.size() != 0) {
			int i = to_visit.front();
			to_visit.pop_front();

			if (count[i] != 2) continue;

			result.push_back(i);

			if (a[i+2] == 1) {
				a[i+2] = 0;
				count[i+2]--;
				count[i+1]--;
				count[i]--;
			}

			if (a[i+1] == 1) {
				a[i+1] = 0;
				count[i+1]--;
				count[i]--;
				if (i > 0) {
					count[i-1] -= 1;
				}
			}

			if (a[i] == 1) {
				a[i] = 0;
				count[i]--;
				if (i > 1) {
					count[i-2] -= 1;
					if (count[i-2] == 2) {
						to_visit.push_back(i-2);
					}
				}
				if (i > 0) {
					count[i-1] -= 1;
					if (count[i-1] == 2) {
						to_visit.push_back(i-1);
					}
				}
			}

			// debug_arr(to_string(i) + ", a", a);
			// debug_arr(to_string(i) + ", count", count);
		}

		count.pop_back();
		count.pop_back();

		// debug_arr("count", count);

		// cout << "!!!" << endl;

		if (count == vector<int>(n-2, 0)) {
			cout << "YES" << endl;
			cout << result.size() << endl;
			for (int i = 0; i < result.size(); ++i) {
				cout << result[i] + 1 << " ";
			}
			cout << endl;
		} else {
			cout << "NO" << endl;
		}
	}
}

/* 

 */