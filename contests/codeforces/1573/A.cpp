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
	for (int i = 0; i < t; ++i) {
		int temp;
		string s;
		cin >> temp;
		cin >> s;
		int count = 0;
		for (int i = 0; i < s.size(); ++i) {
			int number = s[i] - '0';
			if (i == s.size() - 1) {
				count += number;
			} else {
				if (number != 0) {
					count += number + 1;
				}
			}
		}
		cout << count << endl;
	}
}
