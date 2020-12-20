#include <vector>
#include <iostream>
#include <string>
#include <map>
#include <list>
#include <sstream>
#include <fstream>
#include <algorithm>

using namespace std;

typedef int64_t bint;

int main() {
	bint n, k;
	cin >> n >> k;
	vector <pair<bint,bint>> a(n);

	for (int i = 0; i < n; ++i) cin >> a[i].first;
	for (int i = 0; i < n; ++i) cin >> a[i].second;

	bint count = 0;
	vector<int> used(k, 0);
	for (auto& i : a) used[i.first-1]++;
	for (auto& i : used) if (!i) count++;

	sort(a.begin(), a.end(), [](auto a, auto b)->bool { return a.second < b.second; });

	bint sum = 0;
	for (int i = 0; i < n; ++i) {
		if (count == 0) break;

		if (used[a[i].first-1] > 1) {
			used[a[i].first-1]--;
			sum += a[i].second;
			count--;
		}
	}

	cout << sum;
}