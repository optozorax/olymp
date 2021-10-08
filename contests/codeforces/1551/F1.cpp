#include <bits/stdc++.h>
using namespace std;

template <class T> char *debug_arr(T begin, T end) {
	for (; begin != end; ++begin) {
		cerr << *begin << " ";
	}
	return "";
}
template <class T> char *debug_arr2(T begin, T end) {
	for (; begin != end; ++begin) {
		cerr << "{";
        debug_arr((*begin).begin(), (*begin).end());
		cerr << "} ";
	}
	return "";
}
#define DS cerr << "? ";
#define D(A) cerr << #A << ": " << A << ", ";
#define D2(A) cerr << #A << ": " << debug_arr(A.begin(), A.end()) << ", ";
#define D3(A) cerr << #A << ": " << debug_arr2(A.begin(), A.end()) << ", ";
#define DL cerr << endl;

pair<int, int> solve(int a) {
	// if (a == 1) return {1, 0};
	// if (a == 2) return {0, 2};

	int c = a / 3;

	int c1 = c;
	int c2 = c;

	if (a - c1 + 2*c2 == 1) {
		c1 += 1;
	}
	if (a - c1 + 2*c2 == 2) {
		c2 += 2;
	}

	return {c1, c2};
}

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int a;
		cin >> a;
		auto b = solve(a);
		cout << b.first << " " << b.second << endl;
	}
}

/* 

 */
