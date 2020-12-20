#include <iostream>
#include <iomanip>
#include <vector>
#include <bitset>
#include <cmath>
#include <limits>

using namespace std;

typedef int64_t bint;

struct planet
{
	double x, y, z;
};

double dist(planet a, planet b) {
	#define sqr(a) ((a)*(a))
	return sqrt(sqr(a.x-b.x) + sqr(a.y-b.y) + sqr(a.z-b.z));
	#undef sqr
}

bint n;
vector<vector<double>> r;
bitset<20> visited;

vector<vector<double>> cache;

double sum = 0;
int deep = 1;

double travel(int current) {
	auto& c = cache[current][visited.to_ulong()];
	if (c == -1)  {
		double min = numeric_limits<double>::max();
		for (int i = 0; i < n; ++i) {
			if (!visited[i]) {
				visited[i] = true;
				deep++;
				if (deep == n) {
					min = r[current][i];
				} else {
					double sum = r[current][i] + travel(i);
					if (sum < min) min = sum;
				}
				deep--;
				visited[i] = false;
			}
		}
		c = min;
		return c;
	} else
		return c;
}

int main() {
	vector<planet> ps;
	cin >> n;
	for (int i = 0; i < n; i++) {
		planet p;
		cin >> p.x >> p.y >> p.z;
		ps.push_back(p);
	}

	/*n = 4;
	ps.push_back({0, 0, 0});
	ps.push_back({5, 0, 0});
	ps.push_back({5, 1, 0});
	ps.push_back({10, 10, 3});*/

	/*n = 16;
	ps.push_back({ 0, 0, 0 });
	ps.push_back({ 5, 0, 0 });
	ps.push_back({ 5, 1, 0 });
	ps.push_back({ 5, 2, 0 });
	ps.push_back({ 2, 3, 0 });
	ps.push_back({ 5, 1, 5 });
	ps.push_back({ 5, 0, 8 });
	ps.push_back({ 0, 1, 5 });
	ps.push_back({ 5, 1, 4 });
	ps.push_back({ 4, 0, 1 });
	ps.push_back({ 3, 2, 0 });
	ps.push_back({ 5, 0, 5 });
	ps.push_back({20, 30, 50});
	ps.push_back({20, 0, 100});
	ps.push_back({100, 0, 0});
	ps.push_back({300, 20, 20});*/

	/*n = 13;
	ps.push_back({ 0, 0, 0 });
	ps.push_back({ 5, 0, 0 });
	ps.push_back({ 5, 1, 0 });
	ps.push_back({ 5, 2, 0 });
	ps.push_back({ 2, 3, 0 });
	ps.push_back({ 5, 1, 5 });
	ps.push_back({ 5, 0, 8 });
	ps.push_back({ 0, 1, 5 });
	ps.push_back({ 4, 0, 1 });
	ps.push_back({ 3, 2, 0 });
	ps.push_back({ 5, 0, 5 });
	ps.push_back({20, 30, 50});
	ps.push_back({20, 0, 100});*/

	cache.resize(n, vector<double>(pow(2, n), -1));
	r.resize(n, vector<double>(n, 0));
	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < n; ++j) {
			r[i][j] = dist(ps[i], ps[j]);
		}
	}

	visited[0] = true;
	auto result = travel(0);

	cout << setprecision(3) << fixed;
	cout << result;

	//system("pause");
}