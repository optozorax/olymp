#include <algorithm>
#include <iostream>
#include <vector>
#include <set>
#include <queue>
#include <cmath>
 
using namespace std;

template<class T>
void debug_arr(string name, vector<T> a) {
	cout << "| ";
	cout << name << ": ";
	for (int i = 0; i < a.size(); ++i) {
		cout << a[i] << " ";
	}
	cout << endl;
}

#define DBG(A) #A << ": " << A << ", "

int main() {
	vector<string> powers;
	for (int64_t i = 0; i < 63; ++i) {
		powers.push_back(to_string(int64_t(1) << i));
	}

	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		string k;
		cin >> k;

		int my_min = 33;
		for (const auto& s : powers) {
			int i1 = 0, i2 = 0;
			while (i1 < k.size() && i2 < s.size()) {
				if (k[i1] == s[i2]) {
					i1++;
					i2++;
				} else {
					i1++;
				}
			}
			int to_add = s.size() - i2;
			int to_delete = i1 - i2;
			int to_delete_after = k.size() - i1;
			int count = to_add + to_delete + to_delete_after;
			// cout << DBG(s) << DBG(i1) << DBG(i2) << DBG(count) << DBG(to_add) << DBG(to_delete) << DBG(to_delete_after) << endl;
			my_min = min(my_min, count);
		}

		cout << my_min << endl;
	}
}

/* 

34359738368,
687194767
68719476736
549755813888,
576460752303423488,
4611686018427387904,
 */