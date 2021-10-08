#include <algorithm>
#include <iostream>
#include <vector>
#include <set>
#include <queue>
#include <cmath>
 
using namespace std;

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int k;
		cin >> k;
		int pos = int(trunc(sqrt(double(k)-0.5)));
		int top_number = pos * pos + 1;
		k -= top_number;

		int x, y;
		if (k > pos) {
			x = pos - (k - pos);
			y = pos;
		} else {
			x = pos;
			y = k;
		}
		cout << y + 1 << " " << x + 1 << endl;
	}
}

/* 

 */