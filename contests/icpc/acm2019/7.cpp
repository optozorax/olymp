#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <cassert>
#include <optional>
#include <map>
#include <unordered_map>

using namespace std;
typedef int64_t bint;

void test(string str) {
	auto s = new string(str);
	auto iss = new istringstream(*s);
	cin.rdbuf(iss->rdbuf());
}

typedef pair<bool, string> Q;

Q zamena(const string& b, int k, const string& a) {
	vector<char> z(255, 0);
	vector<bool> zanyato(255, false);
	for (int i = 0; i < a.size(); i++) {
		if (z[a[i]] == 0 && !zanyato[b[i+k]]) {
			z[a[i]] = b[i+k];
			zanyato[b[i + k]] = true;
		} else {
			if (b[i+k] != z[a[i]]) {
				return { false, "" };
			}
		}
	}

	string r;
	for (auto& i : z) {
		if (i != 0)
			r += i;
	}
	return { true, r };
}

int main() {
	/*assert(zamena("aabbcb", 2, "aaba") == Q(true, string("bc")));
	assert(zamena("bbcb", 0, "aaba") == Q(true, string("bc")));
	assert(zamena("ythd", 0, "dabc") == Q(true, string("thdy")));
	assert(zamena("bbcd", 0, "aaba") == Q(false, string("")));
	assert(zamena("bacb", 0, "aaba") == Q(false, string("")));
	assert(zamena("bc", 0, "aa") == Q(false, string("")));*/

	//test("10 2 5\nababababab\nba"); // yes 5
	//test("10 2 5\nababababab\naa"); // no 5
	
	bint n, m, l;
	//string ciph(100000, ' '), name(1000, ' '); l = 3;
	string ciph, name;
	cin >> n >> m >> l;
	cin >> ciph >> name;
	/*n = 100000;
	m = 1000;
	for (int i = 0; i < n; i++) {
		if (rand() % 2 == 0) {
			ciph[i] = 'a';
		} else {
			ciph[i] = 'b';
		}
	}
	for (int i = 0; i < m; i++) {
		if (rand() % 2 == 0) {
			name[i] = 'a';
		}
		else {
			name[i] = 'b';
		}
	}*/

	unordered_map<string, bint> p;
	for (int i = 0; i < n-m+1; i++) {
		auto res = zamena(ciph, i, name);
		if (res.first) {
			p[res.second]++;
		}
	}

	for (auto& i : p) {
		if (i.second == l) {
			cout << "YES";
			return 0;
		}
	}
	cout << "NO";
}