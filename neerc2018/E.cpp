#include <vector>
#include <iostream>
#include <string>
#include <map>
#include <list>
#include <sstream>
#include <fstream>
#include <algorithm>

using namespace std;
vector<string> spiral = { "a1", "b1", "b2", "a2", "a3", "b3", "c3", "c2", "c1", "d1", "d2", "d3", "d4", "c4", "b4", "a4", "a5", "b5", "c5", "d5", "e5", "e4", "e3", "e2", "e1", "f1", "f2", "f3", "f4", "f5", "f6", "e6", "d6", "c6", "b6", "a6", "a7", "b7", "c7", "d7", "e7", "f7"};
vector<string> up = {"f8", "a8", "b8", "c8", "d8", "e8", "g8"};
vector<string> v = { "g7", "g6", "g5", "g4", "g3", "g2", "g1"};
vector<string> h = { "h1", "h2", "h3", "h4", "h5", "h6", "h7"};

int main() {
	int n;
	cin >> n;
	if (n == 50) {
		vector<string> loc = {"g7", "g6", "g5", "g4", "g3", "g2", "g1", "h1", "h8"};
		for (auto i : spiral) cout << i << " ";
		for (auto i : loc) cout << i << " ";
	} else if (n <= spiral.size()+1) {
		for (int i = 0; i <= n-2; i++) cout << spiral[i] << " ";
		string end = spiral[n-2];
		end[1] = '8';
		cout << end << " h8";
	} else if (n <= spiral.size()+up.size()+1) {
		for (auto i : spiral) cout << i << " ";	n -= spiral.size();
		for (int i = 0; i < n; i++) cout << up[i] << " ";
		cout << "h8";
	} else if (n <= spiral.size() + up.size() + v.size() + 1) {
		for (auto i : spiral) cout << i << " ";	n -= spiral.size();
		for (auto i : up) cout << i << " ";		n -= up.size();
		for (int i = 0; i < n-1; i++) cout << v[i] << " ";
		string end = v[n-2];
		end[0] = 'h';
		cout << end << " h8";
	} else {
		n += 1;
		for (auto i : spiral) cout << i << " ";	n -= spiral.size();
		for (auto i : up) cout << i << " "; 	n -= up.size();
		for (auto i : v) cout << i << " "; 		n -= v.size();
		for (int i = 0; i < n-1; i++) cout << h[i] << " ";
		cout << "h8";
	}
}