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
		int a, b, c;
		cin >> a >> b >> c;
		int dist = abs(b - a);
		if (c > dist * 2 || a > dist * 2 || b > dist * 2) {
			cout << -1 << endl;
		} else {
			if (c < dist + 1) {
				cout << c + dist << endl;
			} else {
				cout << c - dist << endl;
			}
		}
	}
}

/* 

 */