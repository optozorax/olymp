#include <algorithm>
#include <iostream>
#include <vector>
#include <set>
#include <queue>
#include <map>
#include <cmath>
 
using namespace std;

template<class T>
char* debug_arr(T begin, T end) {
	for (; begin != end; ++begin) {
		cout << *begin << " ";
	}
	return "";
}

#define D(A) #A << ": " << A << ", "
#define D2(A) #A << ": " << debug_arr(A.begin(), A.end()) << ", "

int main() {
	// int t = 1;
	int t = 1000000;
	// int t;
	// cin >> t;
	string previous = "0";
	for (int t1 = 0; t1 < t; ++t1) {
		// string n = to_string((rand() % 10000) * 10000 + rand() % 10000 + 2);
		// int k = rand() % 10 + 1;
		string n = to_string(t1 + 1);
		int k = 3;
		// string n = "12023";
		// int k = 3;
		// string n;
		// int k;
		// cin >> n >> k;

		cout << n << ": ";
		// cout << D(n) << D(k) << endl;

		set<set<int>> visited;
		vector<set<int>> num_nums;
		string r_rs;
		bool first = true;
		bool invalid = false;

		set<int> nums;
		map<int, int> used;
		// for (int i = 0; i < k; ++i) {
		// 	nums.insert(i);
		// }
		bool is_greater = false;
		string r;
		int n_pos = 0;
		while (true) {
			int current = n[n_pos] - '0';

			if (nums.size() < k) {
				nums.insert(current);
			}

			int to_set = -1;
			for (const auto& i : nums) {
				if (is_greater) {
					to_set = i;
					break;
				} else {
					if (i >= current) {
						to_set = i;
						break;
					}	
				}
			}

			// cout << D(n_pos) << D(current) << D2(nums) << D(is_greater) << D(to_set) << D(r) << endl;

			if (to_set != -1) {
				used[to_set] += 1;
				r += to_set + '0';
				if (to_set > current) {
					is_greater = true;
				}
			} else {
				visited.insert(nums);

				for (int i = 0; i < nums.size(); ++i) {
					set<int> to_push;
					for (int j = 0; j < nums.size(); ++j) {
						auto it = nums.begin();
						advance(it, j);
						int current = *it;
						if (j == i) {
							current++;
						}
						to_push.insert(current);
					}
					if (visited.find(to_push) == visited.end()) {
						num_nums.push_back(to_push);
					}
				}

				int min1 = *nums.begin();
				int min_pos = r.size();
				int max_left = 0;
				for (int i = 0; i < r.size(); ++i) {
					int pos = r.size()-1-i;
					char current = r[pos];

					int left = r.find(current);
					if (left > max_left) {
						max_left = left;
						min_pos = pos;
					}
				}
				int pos = min_pos;

				// cout << D(r) << D(min1) << D(pos) << endl;

				int previous = r[pos] - '0';

				if (used[previous] == 1) {
					used[previous] = 0;
					nums.erase(nums.find(previous));
					nums.insert(0);
				}

				int to_set = -1;
				for (const auto& i : nums) {
					if (i > previous) {
						to_set = i;
						break;
					}	
				}
				if (to_set == -1) {
					invalid = true;
				}
				r[pos] = to_set + '0';
				is_greater = true;

				while (pos + 1 != r.size()) {
					r.pop_back();
					n_pos--;
				}
			}

			n_pos++;

			if (r.size() == n.size()) {
				if (first) {
					r_rs = r;
					first = false;
				}

				r_rs = min(r_rs, r);

				invalid = true;
			}

			if (invalid) {
				invalid = false;
				if (num_nums.size() != 0) {
					nums = num_nums.back();
					num_nums.pop_back();
					visited.insert(nums);
					is_greater = false;
					n_pos = 0;
					r.clear();
					used.clear();
					continue;
				} else {
					break;
				}
			}
		}

		cout << r_rs << endl;
		if (r_rs.size() < previous.size() || (r_rs.size() == previous.size() && r_rs < previous)) {
			cout << "AAAAAAAAAAAAA" << endl;
			return 0;
		}
		previous = r_rs;
	}
}

/* 

 */