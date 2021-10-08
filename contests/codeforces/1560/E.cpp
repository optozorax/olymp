#include <bits/stdc++.h>
using namespace std;

template <class T> char *debug_arr(T begin, T end) {
	for (; begin != end; ++begin) {
		cerr << *begin << " ";
	}
	return "";
}
#define DS cerr << "? ";
#define D(A) cerr << #A << ": " << A << ", ";
#define D2(A) cerr << #A << ": " << debug_arr(A.begin(), A.end()) << ", ";
#define DL cerr << endl;

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		string n;
		cin >> n;

		map<char, int> count;
		string order;
		for (int i = 0; i < n.size(); ++i) {
			char current = n[n.size()-1 - i];
			if (count[current] == 0) {
				order.insert(0, 1, current);
			}
			count[current] += 1;
		}

		int size = 0;
		bool invalid = false;
		for (int i = 0; i < order.size(); ++i) {
			int all_count = count[order[i]];
			if (all_count % (i+1) != 0) {
				invalid = true;
				break;
			}
			int repeats_in_original = count[order[i]] / (i+1);
			size += repeats_in_original;
		}

		if (invalid) {
			cout << -1 << endl;
			continue;
		}

		string answer = n.substr(0, size);
		n.erase(0, answer.size());

		string answer_clone = answer;
		for (int i = 0; i < order.size(); ++i) {
			char to_remove = order[i];
			// DS D(answer_clone) DL
			auto end = remove_if(answer_clone.begin(), answer_clone.end(), [&](auto x){ return x == to_remove; });
			answer_clone.erase(end, answer_clone.end()); // stupid c++
			// DS D(answer_clone) D(n) DL
			if (n.substr(0, answer_clone.size()) == answer_clone) {
				n.erase(0, answer_clone.size());
				continue;
			} else {
				invalid = true;
				break;
			}
		}
		if (invalid) {
			cout << -1 << endl;
			continue;
		}

		cout << answer << " " << order << endl;
	}
}
