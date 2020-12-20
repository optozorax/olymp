#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <map>
#include <cmath>
#include <iomanip>

using namespace std;
typedef int64_t bint; 

void test(string str) {
	auto s = new string(str);
	auto iss = new istringstream(*s);
	cin.rdbuf(iss->rdbuf());
}

pair<bint, bint> sort(bint a, bint b) {
	if (a < b) return { a, b };
	else return { b, a };
}

double sqr(double a) {
	return a * a;
}

double dist(pair<bint, bint>& a, pair<bint, bint>& b) {
	return sqrt(sqr(a.first - b.first) + sqr(a.second - b.second));
}

int main() {
	//test("4 2\n0 0\n1 0\n1 1\n0 1\n1 2 3\n1 4 3"); // 4
	//test("4 2\n1 1\n2 1\n2 2\n1 2\n1 2 3\n1 4 3"); // 4
	//test("6 5\n1 1\n2 1\n3 2\n2 3\n1 2\n2 2\n1 2 6\n2 3 6\n3 6 4\n4 5 6\n1 5 6"); // 6.242
	bint n, m;
	vector<pair<bint, bint>> p;
	map<pair<bint, bint>, bint> l;

	cin >> n >> m;
	for (int i = 0; i < n; i++) {
		bint x, y;
		cin >> x >> y;
		p.push_back({ x, y });
	}
	for (int i = 0; i < m; i++) {
		bint a, b, c;
		cin >> a >> b >> c;
		l[sort(a, b)]++;
		l[sort(a, c)]++;
		l[sort(b, c)]++;
	}

	double sum = 0;
	for (auto& i : l) {
		if (i.second == 1) {
			sum += dist(p[i.first.first - 1], p[i.first.second - 1]);
		}
	}

	cout << fixed << setprecision(3) << sum;
}