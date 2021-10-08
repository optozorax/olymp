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
		int k;
		cin >> k;
		int i = 1;
		while (k != 0) {
			if (i % 10 != 3 && i % 3 != 0) k--;
			i++;
		}
		cout << i-1 << endl;
	}
}

/* 

 */