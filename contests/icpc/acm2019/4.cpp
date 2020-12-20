#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <cassert>
#include <optional>
#include <map>
#include <unordered_map>
#include <set>
#include <algorithm>
#include <functional>

using namespace std;
typedef int64_t bint;

void test(string str) {
	auto s = new string(str);
	auto iss = new istringstream(*s);
	cin.rdbuf(iss->rdbuf());
}

pair<bint, bint> sort1(bint a, bint b) {
	if (a < b) return {a, b};
	else return {b, a};
}

map<pair<bint, bint>, bint> t;
vector<vector<bint>> neighbors;
set<bint> locked;
vector<vector<bint>> m;
bint dist(bint a, bint b) {
	auto& result = m[a][b];
	if (result != -1)
		return result;

	auto& semi_result = m[b][a];
	if (semi_result != -1) {
		result = semi_result;
		return result;
	}

	if (a == b) {
		result = 0;
		return result;
	}

	locked.insert(a);

	const bint max_dist = 1'000'000'000;
	bint min_dist = max_dist;
	auto& adjacent = neighbors[a];
	for (auto& i : adjacent) {
		if (locked.find(i) == locked.end()) {
			auto res = dist(i, b);
			if (res != max_dist) {
				min_dist = min(min_dist, res + t[sort1(a, i)]);
			}
		}
	}

	locked.erase(a);

	if (min_dist != max_dist)
		result = min_dist;
	return min_dist;
}

int main() {
	//test("7 4\n0 1\n1 2\n2 1\n2 1\n2 2\n1 2"); // 5
	//test("7 1\n0 1\n1 2\n2 1\n2 1\n2 2\n1 2"); // 2
	bint n, l;
	cin >> n >> l;

	neighbors.resize(n);
	m.resize(n, vector<bint>(n, -1));

	for (int i = 0; i < n-1; ++i) {
		bint boss, time;
		cin >> boss >> time;
		bint current = i+1;
		neighbors[boss].push_back(current);
		neighbors[current].push_back(boss);
		t[sort1(boss, current)] = time;
	}

	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < n; ++j) {
			if (m[i][j] == -1) {
				dist(i, j);
			}
		}
	}

	// for (auto& i : m) {
	// 	for (auto& j : i) {
	// 		cout << j << " ";
	// 	}
	// 	cout << endl;
	// }

	// Неудачный алгоритм поиска поддерева
	// bint max_ = 0;
	// for (int i = 0; i < n; ++i) {
	// 	bint count = 0;
	// 	for (int j = 0; j < n; ++j) {
	// 		if (m[i][j] > 0 && m[i][j] <= l) {
	// 			count++;
	// 		}
	// 	}
	// 	max_ = max(max_, count);
	// }

	vector<bint> bad_count(n, 0);
	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < n; ++j) {
			bad_count[i] += m[i][j] > l;
		}
	}

	bint max_ = n;
	while (true) {
		auto max_bad = max_element(bad_count.begin(), bad_count.end());
		if (*max_bad == 0) break;
		max_--;

		bint indx = max_bad - bad_count.begin();
		for (int i = 0; i < n; ++i) {
			{
				auto& p = m[i][indx];
				if (p > l) {
					p = 0;
					bad_count[i]--;
				}
			}
			{
				auto& p = m[indx][i];
				if (p > l) {
					p = 0;
					bad_count[indx]--;
				}
			}
		}
	}

	cout << max_;
}