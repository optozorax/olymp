#include <iostream>
#include <string>
#include <vector>

using namespace std;

typedef int64_t bint;

int main() {
	vector<int> r;
	string s;
	cin >> s;

	for (int i = 0; i < s.size(); i++) {
		char c = s[i];
		int j = i+1;
		for (; j < s.size(); j++) {
			if (s[j] != c)
				break;
		}
		r.push_back(j - i);
		i = j-1;
	}

	cout << r.size() << endl;
	for (auto& i : r)
		cout << i << " ";
}