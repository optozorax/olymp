#include <algorithm>
#include <cmath>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <vector>

using namespace std;

template <class T> char *debug_arr(T begin, T end) {
	for (; begin != end; ++begin) {
		cout << *begin << " ";
	}
	return "";
}
#define D(A) #A << ": " << A << ", "
#define D2(A) #A << ": " << debug_arr(A.begin(), A.end()) << ", "

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		string n;
		int k;
		cin >> n >> k;

		while (true) {
			set<char> nums;
			int pos = 0;
			for (; pos < n.size(); ++pos) {
				nums.insert(n[pos]);
				if (nums.size() == k + 1) {
					nums.erase(nums.find(n[pos]));
					pos--;
					break;
				}
			}

			if (pos == n.size()) {
				break;
			}

			pos++;

			int pos2 = pos;
			while (n[pos2] == '9') {
				n[pos2] = '0';
				pos2--;
			}
			n[pos2] += 1;

			for (pos++; pos < n.size(); ++pos) {
				n[pos] = '0';
			}
		}

		cout << n << endl;
	}
}