#include<iostream>
#include<vector>
#include<algorithm>
#include<functional>
using namespace std;
int main() {
	int n; char c;
	cin >> n;
	vector<int> d('z'-'a'+1,0);
	for (int i = 0; i < n; ++i) { cin >> c; d[c-'a']++; }
	for (int i = 0; i < n; ++i) { cin >> c; d[c-'a']++; }
	sort(d.begin(), d.end(), greater<int>());
	int m = 0, i = 0;
	for (; i < d.size(); ++i) {
		m += d[i];
		if (m >= n) 
			break;
	}
	cout << i+1;
}