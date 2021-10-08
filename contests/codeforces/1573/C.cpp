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

		vector<vector<int>> adj(n);
		vector<vector<int>> adj2(n);
		for (int i = 0; i < n; ++i) {
			int k;
			cin >> k;
			adj[i].resize(k, 0);
			for (int j = 0; j < k; ++j) {
				int temp;
				cin >> temp;
				adj[i][j] = temp - 1;
				adj2[temp-1].push_back(i);
			}
		}

		vector<int> count(n, 0);
		vector<int> iter(n, 0);

		deque<int> to_visit;
		for (int i = 0; i < n; ++i) {
			count[i] = adj[i].size();
			if (count[i] == 0) {
				to_visit.push_back(i);
			}
		}

		// debug_arr("count: ", count);

		while (to_visit.size() != 0) {
			int current = to_visit.front();
			to_visit.pop_front();
			for (const auto& i : adj2[current]) {
				count[i]--;
				if (i < current) {
					iter[i] = max(iter[i], iter[current] + 1);
				} else {
					iter[i] = max(iter[i], iter[current]);
				}
				if (count[i] == 0) {
					to_visit.push_back(i);
				}
			}

			// debug_arr(to_string(current) + ", count: ", count);
			// debug_arr(to_string(current) + ", iter: ", iter);
			// cout << endl;
		}

		// cout << "! ";
		if (count == vector<int>(n, 0)) {
			cout << *max_element(iter.begin(), iter.end()) + 1 << endl;
		} else {
			cout << -1 << endl;
		}
	}
}

/* 

 */